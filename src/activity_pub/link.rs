use activitystreams_kinds::link::LinkType;
use actix_web::{body::BoxBody, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    r#type: LinkType,
    href: String,
    media_type: String,
}

impl Link {
    pub fn jpeg(href: impl Into<String>) -> Self {
        Self {
            r#type: LinkType::Link,
            href: href.into(),
            media_type: "image/jpeg".into(),
        }
    }
}

impl actix_web::Responder for Link {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(body)
    }
}
