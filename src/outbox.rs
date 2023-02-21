use activitystreams_kinds::collection::OrderedCollectionType;
use actix_web::{body::BoxBody, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
    r#type: OrderedCollectionType,
    id: String,
    items: Vec<()>,
    total_items: usize,
}

impl Outbox {
    pub fn empty(id: String) -> Self {
        Self {
            r#type: OrderedCollectionType::OrderedCollection,
            id,
            items: vec![],
            total_items: 0,
        }
    }
}

impl actix_web::Responder for Outbox {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(body)
    }
}
