//! Checkout the [diesel webpage](https://diesel.rs) for
//! longer guides about diesel
//!
//! Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
//! for a real world application using axum and diesel
#[macro_use]
extern crate tracing;
use axum::extract::{FromRef, FromRequestParts, State};
use database_pools::ConnectionConfig;
use deadpool_diesel::postgres::{Manager as DeadpoolManager, Pool as DeadpoolPool};
use deadpool_diesel::Runtime;
use menva::get_env;
use middlewares::auth::Keys;
use server_config::Server;
use std::ops::Deref;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};

mod database_pools;

mod middlewares;

mod responses;
mod router;
mod schema;
mod server_config;
mod users;
mod versioning;
use router::build_router;

const CORE_THREADS: usize = 4;

type DeadpoolResult = Result<deadpool_diesel::postgres::Connection, deadpool_diesel::PoolError>;

pub struct App {
    /// Database connection pool connected to the primary database
    pub primary_database: DeadpoolPool,

    /// Database connection pool connected to the read-only replica database
    pub replica_database: Option<DeadpoolPool>,

    pub keys: Keys,
}
fn maybe_append_url_param(url: &mut url::Url, key: &str, value: &str) {
    if !url.query_pairs().any(|(k, _)| k == key) {
        url.query_pairs_mut().append_pair(key, value);
    }
}

fn connection_url(config: &database_pools::DatabasePools, url: &str) -> String {
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
    pub fn new(config: &Server) -> App {
        let primary_database = {
            let primary_db_connection_config = ConnectionConfig {
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
            let replica_db_connection_config = ConnectionConfig {
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
        Self {
            keys: Keys::new(get_env("SESSION_KEY").as_bytes()),
            primary_database,
            replica_database,
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

fn main() -> Result<(), i16> {
    menva::read_default_file();
    let config = Server::from_environment();

    let app = Arc::new(App::new(&config));

    // Start the background thread periodically logging instance metrics.
    // log_instance_metrics_thread(app.clone());
    let state = AppState(app);
    info!("lets go!");
    let axum_router = build_router(state.clone());

    // Apply the `normalize_path` middleware around the axum router.
    //
    // See https://docs.rs/axum/0.7.2/axum/middleware/index.html#rewriting-request-uri-in-middleware.
    // let normalize_path = axum::middleware::from_fn(normalize_path);
    // let axum_router = normalize_path.layer(axum_router);

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.worker_threads(CORE_THREADS);
    // if let Some(threads) = app.config.max_blocking_threads {
    //     builder.max_blocking_threads(threads);
    // }

    let rt = builder.build().unwrap();

    let make_service = axum_router.into_make_service_with_connect_info::<SocketAddr>();

    // Block the main thread until the server has shutdown
    rt.block_on(async {
        // Create a `TcpListener` using tokio.
        // let listener = TcpListener::bind((app.config.ip, app.config.port)).await?;
        let listener = TcpListener::bind(("127.0.0.1", 8000)).await?;
        info!("Server started!");

        let addr = listener.local_addr()?;

        // Do not change this line! Removing the line or changing its contents in any way will break
        // the test suite :)
        info!("Listening at http://{addr}");

        // Run the server with graceful shutdown
        axum::serve(listener, make_service)
            .with_graceful_shutdown(shutdown_signal())
            .await
    })
    .unwrap();

    info!("Server has gracefully shutdown!");
    Ok(())
}

async fn shutdown_signal() {
    let interrupt = async {
        signal(SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = interrupt => {},
        _ = terminate => {},
    }
}

// fn log_instance_metrics_thread(app: Arc<App>) {
//     // Only run the thread if the configuration is provided
//     let interval = match app.config.instance_metrics_log_every_seconds {
//         Some(secs) => Duration::from_secs(secs),
//         None => return,
//     };

//     std::thread::spawn(move || loop {
//         if let Err(err) = log_instance_metrics_inner(&app) {
//             error!(?err, "log_instance_metrics error");
//         }
//         std::thread::sleep(interval);
//     });
// }
