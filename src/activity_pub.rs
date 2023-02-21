mod create;
mod image;
mod link;
mod outbox;

use actix_web::{
    web::{self, get},
    HttpRequest, HttpResponse, Responder, Result,
};
use create::Create;
use image::Image;
use link::Link;
use outbox::Outbox;

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

async fn outbox(req: HttpRequest) -> Result<impl Responder> {
    let mut outbox = Outbox::empty(req.url_for_static("outbox")?);

    let date = "20230218";
    let time = "02:00:00";
    let link = Link::jpeg(req.url_for("image", &[date, date, time])?);
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(req.url_for_static("actor")?, image);
    outbox.push(create);

    let date = "20230218";
    let time = "01:00:00";
    let link = Link::jpeg(req.url_for("image", &[date, date, time])?);
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(req.url_for_static("actor")?, image);
    outbox.push(create);

    let date = "20230218";
    let time = "00:00:00";
    let link = Link::jpeg(req.url_for("image", &[date, date, time])?);
    let image = Image::new(format!("{date}-{time}"), vec![link]);
    let create = Create::new(req.url_for_static("actor")?, image);
    outbox.push(create);

    Ok(outbox)
}

async fn profile() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<!DOCTYPE html>
<title>About hourly weather</title>

<p>Hourly weather photos from Seattle
"#,
    )
}

pub fn app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("actor").route(get().to(history)));
    cfg.service(web::resource("/about").route(get().to(profile)));
    cfg.service(
        web::resource("/outbox")
            .name("outbox")
            .route(get().to(outbox)),
    );
    cfg.external_resource(
        "image",
        "https://weather.segment7.net/images/{date1}/{date2}-{time}.jpeg",
    );
}
