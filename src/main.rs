mod companies;
mod countries;
mod currencies;
mod db;
mod dictionary;
mod exchanges;
mod industries;
mod sectors;
mod server;
mod transactions;
mod users;

#[macro_use]
extern crate tracing;

use menva::read_default_file;

use server::{get_router, init_dev_tracing, init_prod_tracing, App, AppState, Config, EnvIs};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
};

const CORE_THREADS: usize = 4;

fn main() -> Result<(), i16> {
    read_default_file();

    let config = Config::from_environment();
    match config.env {
        EnvIs::Dev => init_dev_tracing(),
        EnvIs::Prod => init_prod_tracing(),
    }

    let app = Arc::new(App::new(&config));
    let state = AppState(app);
    info!("lets go!");
    let router = get_router(state.clone());

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all().worker_threads(CORE_THREADS);

    if let Some(threads) = config.max_blocking_threads {
        builder.max_blocking_threads(threads);
    }

    let rt = builder.build().unwrap();

    let service = router.into_make_service_with_connect_info::<SocketAddr>();

    // Block the main thread until the server has shutdown
    rt.block_on(async {
        let listener = TcpListener::bind((config.ip, config.port)).await?;

        axum::serve(listener, service)
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
