mod activity_pub;
mod args;
mod hourly;
mod hourly_weather;
mod root;
mod tracing;
mod webfinger;

pub use crate::args::Args;
use crate::root::root;
pub use crate::webfinger::webfinger;
use clap::Parser;
pub use hourly_weather::HourlyWeather;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing::init("hourly-weather");

    let args = Args::parse();

    let state = HourlyWeather::new("weather.segment7.net");
    let app = root(&args).with_state(state);

    let tls_config = args.tls_config().await;

    let port = 8443;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
