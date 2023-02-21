mod tracing;
mod webfinger;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
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

#[get("/hourly")]
async fn profile() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<!DOCTYPE html>
<title>Hourly weather</title>

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
            .service(profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
