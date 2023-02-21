use actix_files::NamedFile;
use actix_web::{
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
    cfg.service(web::resource("/").route(get().to(empty)));
    cfg.service(web::resource("/sky.jpeg").route(get().to(sky_jpeg)));
}
