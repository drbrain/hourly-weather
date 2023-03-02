use crate::activity_pub::{create::Create, Context};
use activitystreams_kinds::collection::OrderedCollectionType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
    #[serde(rename = "@context")]
    context: Context,
    r#type: OrderedCollectionType,
    id: String,
    items: Vec<Create>,
    total_items: usize,
}

impl Outbox {
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            context: Context::activitystreams(),
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
