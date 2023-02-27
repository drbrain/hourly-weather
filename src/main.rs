mod activity_pub;
mod args;
mod hourly;
mod tracing;
mod webfinger;

use crate::{hourly::app, webfinger::webfinger};
use axum::{routing::get, Router};
use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug)]
pub struct HourlyWeather {
    domain: String,
}

impl HourlyWeather {
    fn new(domain: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            domain: domain.into(),
        })
    }

    pub fn actor(&self) -> String {
        format!("https://{}/hourly", self.domain)
    }

    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    pub fn image(&self, date: &str, time: &str) -> String {
        format!("https://{}/images/{date}/{date}-{time}.jpeg", self.domain)
    }

    pub fn outbox(&self) -> String {
        format!("https://{}/hourly/outbox", self.domain)
    }
}

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
