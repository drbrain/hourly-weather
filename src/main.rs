mod activity_pub;
mod args;
mod host_meta;
mod hourly;
mod hourly_weather;
mod http;
mod root;
mod tracing;
mod webfinger;

pub use crate::args::Args;
pub use crate::host_meta::host_meta;
pub use crate::webfinger::webfinger;
use clap::Parser;
pub use hourly_weather::HourlyWeather;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing::init();

    http::start(Args::parse()).await.unwrap();

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
