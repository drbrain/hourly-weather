use actix_files::NamedFile;
use actix_web::{
    guard,
    web::{self, get},
    Result,
};

async fn empty() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}
async fn sky_jpeg() -> Result<NamedFile> {
    Ok(NamedFile::open("/data/www/weather/sky.jpeg")?)
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .guard(guard::Header("Host", "weather.segment7.net"))
            .route(get().to(empty)),
    );
    cfg.service(
        web::resource("/sky.jpeg")
            .guard(guard::Header("Host", "weather.segment7.net"))
            .route(get().to(sky_jpeg)),
    );
}
