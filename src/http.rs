use crate::{root::root, Args, HourlyWeather};
use std::{io::Error, net::SocketAddr};
use tokio::{
    signal::{ctrl_c, unix},
    time::Duration,
};
use tracing::{info, warn};

pub async fn start(args: Args) -> Result<(), Error> {
    let state = HourlyWeather::new("weather.segment7.net");
    let app = root(&args).with_state(state);

    let tls_config = args.tls_config().await;

    let port = 8443;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("binding {addr:?}");

    let handle = axum_server::Handle::new();

    let server = axum_server::bind_rustls(addr, tls_config);
    let server = server.handle(handle.clone());

    //tokio::task::spawn(shutdown(handle));

    server
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
}

async fn shutdown(handle: axum_server::Handle) {
    let sigint = async {
        ctrl_c().await.expect("failed to install SIGINT handler");
    };

    let sigterm = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = sigint => {
            warn!("SIGINT received, shutting down");
        },
        _ = sigterm => {
            warn!("SIGTERM received, shutting down");
        },
    }

    opentelemetry::global::shutdown_tracer_provider();

    handle.graceful_shutdown(Some(Duration::from_secs(1)));
}
