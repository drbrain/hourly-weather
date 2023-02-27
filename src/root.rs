use crate::hourly;
use crate::{webfinger, Args, HourlyWeather};
use axum::{body::HttpBody, routing::get, Router};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

async fn healthcheck() {}

pub fn root<B>(args: &Args) -> Router<Arc<HourlyWeather>, B>
where
    B: HttpBody + Send + 'static,
{
    Router::new()
        .route("/.well-known/webfinger", get(webfinger).head(webfinger))
        .route("/healthcheck", get(healthcheck).head(healthcheck))
        .nest("/hourly", hourly::app())
        .nest_service("/images", ServeDir::new(args.images_dir()))
        .route_service("/sky.jpeg", ServeFile::new(args.sky_jpeg()))
}
