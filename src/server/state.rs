use super::{auth::Keys, Config};
use axum::extract::{FromRequestParts, State};
use deadpool_diesel::{
    postgres::{Manager as DeadpoolManager, Pool as DeadpoolPool},
    Runtime,
};
use menva::get_env;
use minijinja::{context, Environment};
use std::{ops::Deref, sync::Arc};

type DeadpoolResult = Result<deadpool_diesel::postgres::Connection, deadpool_diesel::PoolError>;

pub struct App {
    /// Database connection pool connected to the primary database
    pub primary_database: DeadpoolPool,

    /// Database connection pool connected to the read-only replica database
    pub replica_database: Option<DeadpoolPool>,

    pub keys: Keys,
    pub templates: Environment<'static>,
}

fn maybe_append_url_param(url: &mut url::Url, key: &str, value: &str) {
    if !url.query_pairs().any(|(k, _)| k == key) {
        url.query_pairs_mut().append_pair(key, value);
    }
}

fn connection_url(config: &crate::db::DatabasePools, url: &str) -> String {
    let mut url = url::Url::parse(url).expect("Invalid database URL");

    if config.enforce_tls {
        maybe_append_url_param(&mut url, "sslmode", "require");
    }

    // Configure the time it takes for diesel to return an error when there is full packet loss
    // between the application and the database.
    maybe_append_url_param(
        &mut url,
        "tcp_user_timeout",
        &config.tcp_timeout_ms.to_string(),
    );

    url.into()
}

impl App {
    /// Creates a new `App` with a given `Config` and an optional HTTP `Client`
    ///
    /// Configures and sets up:
    ///
    /// - Database connection pools
    pub fn new(config: &super::config::Config) -> App {
        let primary_database = {
            let primary_db_connection_config = crate::db::ConnectionConfig {
                statement_timeout: config.db.statement_timeout,
                read_only: config.db.primary.read_only_mode,
            };

            let url = connection_url(&config.db, &config.db.primary.url);
            let manager = DeadpoolManager::new(url, Runtime::Tokio1);

            DeadpoolPool::builder(manager)
                .runtime(Runtime::Tokio1)
                .max_size(config.db.primary.pool_size)
                .wait_timeout(Some(config.db.connection_timeout))
                .post_create(primary_db_connection_config)
                .build()
                .unwrap()
        };

        let replica_database = if let Some(pool_config) = config.db.replica.as_ref() {
            let replica_db_connection_config = crate::db::ConnectionConfig {
                statement_timeout: config.db.statement_timeout,
                read_only: pool_config.read_only_mode,
            };

            let url = connection_url(&config.db, &pool_config.url);
            let manager = DeadpoolManager::new(url, Runtime::Tokio1);

            let pool = DeadpoolPool::builder(manager)
                .runtime(Runtime::Tokio1)
                .max_size(pool_config.pool_size)
                .wait_timeout(Some(config.db.connection_timeout))
                .post_create(replica_db_connection_config)
                .build()
                .unwrap();

            Some(pool)
        } else {
            None
        };

        let mut templates = Environment::new();
        templates
            .add_template("home", include_str!("../../templates/index.jinja"))
            .unwrap();
        templates
            .add_template("layout", include_str!("../../templates/layout.jinja"))
            .unwrap();

        Self {
            keys: Keys::new(get_env("SESSION_KEY").as_bytes()),
            primary_database,
            replica_database,
            templates,
        }
    }

    /// Obtain a read/write database connection from the async primary pool
    #[instrument(skip_all)]
    pub async fn db_write(&self) -> DeadpoolResult {
        self.primary_database.get().await
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

// deref so you can still access the inner fields easily
impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
