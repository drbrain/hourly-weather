use crate::{host_meta, hourly, webfinger, Args, HourlyWeather};
use axum::{body::HttpBody, routing::get, Router};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

async fn healthcheck() {}

pub fn root<B>(args: &Args) -> Router<Arc<HourlyWeather>, B>
where
    B: HttpBody + Send + 'static,
{
    Router::new()
        .route("/.well-known/host-meta", get(host_meta).head(host_meta))
        .route("/.well-known/webfinger", get(webfinger).head(webfinger))
        .nest("/hourly", hourly::app())
        .nest_service("/images", ServeDir::new(args.images_dir()))
        .route_service("/sky.jpeg", ServeFile::new(args.sky_jpeg()))
        .layer(opentelemetry_tracing_layer())
        .route("/healthcheck", get(healthcheck).head(healthcheck))
}
