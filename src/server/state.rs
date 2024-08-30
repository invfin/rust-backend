use super::{auth::Keys, AppError};
use axum::extract::{FromRequestParts, State};
use deadpool_diesel::{
    postgres::{Manager as DeadpoolManager, Pool as DeadpoolPool},
    Runtime,
};
use menva::get_env;
use std::{ops::Deref, sync::Arc};

pub struct App {
    /// Database connection pool connected to the primary database
    pub primary_database: DeadpoolPool,
    pub ips_database: maxminddb::Reader<Vec<u8>>,
    pub keys: Keys,
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
        let ips_database = maxminddb::Reader::open_readfile(&config.ips_database).unwrap();
        Self {
            keys: Keys::new(get_env("SESSION_KEY").as_bytes()),
            primary_database,
            ips_database,
        }
    }

    /// Obtain a read/write database connection from the async primary pool
    #[instrument(skip_all)]
    pub async fn db_write(&self) -> Result<deadpool_diesel::postgres::Connection, AppError> {
        self.primary_database
            .get()
            .await
            .map_err(AppError::DatabasePoolError)
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
