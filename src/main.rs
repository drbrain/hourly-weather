mod tracing;
mod webfinger;

use activitystreams_kinds::collection::OrderedCollectionType;
use actix_web::{
    body::BoxBody,
    web::{get, scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;
use tracing_actix_web::TracingLogger;
use webfinger::Resolver;

#[derive(Debug)]
pub struct HourlyWeather {
    domain: String,
}

impl HourlyWeather {
    fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
        }
    }

    fn to_app_data(self) -> Data<HourlyWeather> {
        Data::new(self)
    }
}

async fn history() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"<!DOCTYPE html>
<title>Hourly weather history</title>

<ol>
<ul><img src="/images/weather/20230218/20230218-00:00:00.jpeg">
<ul><img src="/images/weather/20230218/20230218-01:00:00.jpeg">
<ul><img src="/images/weather/20230218/20230218-02:00:00.jpeg">
</ul>
"#,
        )
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Outbox {
    r#type: OrderedCollectionType,
    id: String,
    items: Vec<()>,
    total_items: usize,
}

impl Outbox {
    fn empty(id: String) -> Self {
        Self {
            r#type: OrderedCollectionType::OrderedCollection,
            id,
            items: vec![],
            total_items: 0,
        }
    }
}

impl actix_web::Responder for Outbox {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(body)
    }
}

async fn outbox() -> impl Responder {
    Outbox::empty("https://weather.segment7.net/hourly/outbox".into())
}

async fn profile() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<!DOCTYPE html>
<title>About hourly weather</title>

<p>Hourly weather photos from Seattle
"#,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::init("hourly-weather");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(HourlyWeather::new("weather.segment7.net").to_app_data())
            .service(actix_webfinger::resource::<Resolver>())
            .service(
                scope("/hourly")
                    .route("/", get().to(history))
                    .route("/about", get().to(profile))
                    .route("/outbox", get().to(outbox)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
