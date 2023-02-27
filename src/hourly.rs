use std::sync::Arc;

use axum::{
    body::HttpBody,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use hyper::StatusCode;

use crate::{
    activity_pub::{Create, Image, Link, Outbox},
    HourlyWeather,
};

pub fn app<B>() -> Router<Arc<HourlyWeather>, B>
where
    B: HttpBody + Send + 'static,
{
    Router::new()
        .route("/", get(history).head(history))
        .route("/outbox", get(outbox).head(outbox))
        .route("/profile", get(profile).head(profile))
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

async fn outbox(State(state): State<Arc<HourlyWeather>>) -> Result<impl IntoResponse, StatusCode> {
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

    Ok(Json(outbox))
}

async fn profile() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<title>About hourly weather</title>

<p>Hourly weather photos from Seattle
"#,
    )
}
