use crate::HourlyWeather;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;
use std::sync::Arc;

pub async fn host_meta(State(state): State<Arc<HourlyWeather>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "application/xrd+xml; charset=utf-8")],
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
  <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}"/>
</XRD>"#,
            state.domain()
        ),
    )
}
