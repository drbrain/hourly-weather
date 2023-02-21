use crate::activity_pub::create::Create;
use activitystreams_kinds::collection::OrderedCollectionType;
use actix_web::{body::BoxBody, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
    r#type: OrderedCollectionType,
    id: String,
    items: Vec<Create>,
    total_items: usize,
}

impl Outbox {
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            r#type: OrderedCollectionType::OrderedCollection,
            id: id.into(),
            items: vec![],
            total_items: 0,
        }
    }

    pub fn push(&mut self, activity: Create) {
        self.items.push(activity);
        self.total_items = self.items.len();
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
