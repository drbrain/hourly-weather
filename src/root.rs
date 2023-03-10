use crate::{host_meta, hourly, sky_jpeg::sky_jpeg, webfinger, Args, HourlyWeather};
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
use std::sync::Arc;
use tower_http::services::ServeDir;

async fn healthcheck() {}

pub fn root(args: &Args) -> Router<Arc<HourlyWeather>, hyper::Body> {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta).head(host_meta))
        .route("/.well-known/webfinger", get(webfinger).head(webfinger))
        .nest("/hourly", hourly::app())
        .nest_service("/images", ServeDir::new(args.images_dir()))
        .route_service("/sky.jpeg", sky_jpeg(args.sky_jpeg()))
        .layer(response_with_trace_layer())
        .layer(opentelemetry_tracing_layer())
        .route("/healthcheck", get(healthcheck).head(healthcheck))
}
