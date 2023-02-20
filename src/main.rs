mod webfinger;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
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

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("It works!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(HourlyWeather::new("weather.segment7.net").to_app_data())
            .service(actix_webfinger::resource::<Resolver>())
            .service(root)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
