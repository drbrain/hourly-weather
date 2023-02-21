use crate::activity_pub::image::Image;
use activitystreams_kinds::activity::CreateType;
use actix_web::{body::BoxBody, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    r#type: CreateType,
    actor: String,
    object: Image,
}

impl Create {
    pub fn new(actor: impl Into<String>, object: Image) -> Self {
        Self {
            r#type: CreateType::Create,
            actor: actor.into(),
            object,
        }
    }
}

impl actix_web::Responder for Create {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(body)
    }
}
