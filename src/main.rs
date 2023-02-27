mod activity_pub;
mod args;
mod hourly;
mod hourly_weather;
mod tracing;
mod webfinger;

use crate::{hourly::app, webfinger::webfinger};
use axum::{routing::get, Router};
use clap::Parser;
pub use hourly_weather::HourlyWeather;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing::init("hourly-weather");

    let args = args::Args::parse();
    let tls_config = args.tls_config().await;

    let state = HourlyWeather::new("weather.segment7.net");

    let app = Router::new()
        .route("/.well-known/webfinger", get(webfinger))
        .nest("/hourly", app())
        .nest_service("/images", ServeDir::new(args.images_dir()))
        .route_service("/sky.jpeg", ServeFile::new(args.sky_jpeg()))
        .with_state(state);

    let port = 8443;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
