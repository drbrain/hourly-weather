mod create;
mod image;
mod link;
mod outbox;
mod tracing;
mod webfinger;

use actix_web::{
    web::{get, scope, Data},
    App, HttpResponse, HttpServer, Responder,
};
use create::Create;
use image::Image;
use link::Link;
use outbox::Outbox;
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
<ul><img src="/images/20230218/20230218-00:00:00.jpeg">
<ul><img src="/images/20230218/20230218-01:00:00.jpeg">
<ul><img src="/images/20230218/20230218-02:00:00.jpeg">
</ul>
"#,
        )
}

async fn outbox() -> impl Responder {
    let mut outbox = Outbox::empty("https://weather.segment7.net/hourly/outbox".into());
    let link = Link::jpeg("https://weather.segment7.net/images/20230218/20230218-00:00:00.jpeg");
    let image = Image::new("20230218-00:00", vec![link]);
    let create = Create::new("https://weather.segment7.net/hourly", image);
    outbox.push(create);
    outbox
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
            .service(actix_files::Files::new("/images", "images"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
