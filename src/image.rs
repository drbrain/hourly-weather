use activitystreams_kinds::object::ImageType;
use actix_web::{body::BoxBody, HttpResponse};
use serde::Serialize;

use crate::link::Link;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    r#type: ImageType,
    name: String,
    url: Vec<Link>,
}

impl Image {
    pub fn new(name: impl Into<String>, url: Vec<Link>) -> Self {
        Self {
            r#type: ImageType::Image,
            name: name.into(),
            url,
        }
    }
}

impl actix_web::Responder for Image {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(body)
    }
}
