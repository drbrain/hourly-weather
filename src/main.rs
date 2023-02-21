mod activity_pub;
mod args;
mod sky_jpeg;
mod tracing;
mod webfinger;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use clap::Parser;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::init("hourly-weather");

    let args = args::Args::parse();
    let tls_config = args.tls_config();

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(HourlyWeather::new("weather.segment7.net").to_app_data())
            .service(actix_webfinger::resource::<Resolver>())
            .service(scope("/hourly").configure(activity_pub::app))
            .service(actix_files::Files::new("/images", "images"))
            .configure(sky_jpeg::service)
    })
    .bind(("127.0.0.1", 8080))?
    .bind_rustls(("0.0.0.0", 8443), tls_config)?
    .run()
    .await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
