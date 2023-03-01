use crate::{
    activity_pub::{Create, Image, Link, Outbox},
    HourlyWeather,
};
use axum::{
    body::HttpBody,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, http::HeaderValue,
};
use hyper::HeaderMap;
use std::sync::Arc;

pub fn app<B>() -> Router<Arc<HourlyWeather>, B>
where
    B: HttpBody + Send + 'static,
{
    Router::new()
        .route("/", get(root).head(root))
        .route("/about", get(about).head(about))
}

async fn about() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<title>About hourly weather</title>

<p>Hourly weather photos from Seattle
"#,
    )
}

async fn history() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<title>Hourly weather history</title>

<ol>
<ul><img src="/images/20230226/20230226-00:00:00.jpeg">
<ul><img src="/images/20230226/20230226-01:00:00.jpeg">
<ul><img src="/images/20230226/20230226-02:00:00.jpeg">
</ul>
"#,
    )
}

async fn outbox(State(state): State<Arc<HourlyWeather>>) -> Json<Outbox> {
    let mut outbox = Outbox::empty(state.outbox());

    let date = "20230226";
    let time = "02:00:00";
    let link = Link::jpeg(state.image(date, time));
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(state.actor(), image);
    outbox.push(create);

    let date = "20230226";
    let time = "01:00:00";
    let link = Link::jpeg(state.image(date, time));
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(state.actor(), image);
    outbox.push(create);

    let date = "20230226";
    let time = "00:00:00";
    let link = Link::jpeg(state.image(date, time));
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(state.actor(), image);
    outbox.push(create);

    Json(outbox)
}

async fn root(
    State(state): State<Arc<HourlyWeather>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let mut response = if let Some(accept) = headers.get("accept") {
        if accept.to_str().unwrap_or("").contains("application/activity+json") {
            outbox(State(state)).await.into_response()
        } else {
            history().await.into_response()
        }
    } else {
        history().await.into_response()
    };

    let headers = response.headers_mut();
    headers.insert("vary", HeaderValue::from_static("accept"));

    response
}
