use crate::activity_pub::{image::Image, CONTEXT};
use activitystreams_kinds::actor::ServiceType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "@context")]
    context: String,
    r#type: ServiceType,
    id: String,
    name: String,
    icon: Image,
    inbox: String,
    outbox: String,
    preferred_username: String,
}

impl Service {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        icon: Image,
        inbox: impl Into<String>,
        outbox: impl Into<String>,
        username: impl Into<String>,
    ) -> Self {
        Self {
            context: CONTEXT.into(),
            r#type: ServiceType::Service,
            id: id.into(),
            name: name.into(),
            icon,
            outbox: outbox.into(),
            inbox: inbox.into(),
            preferred_username: username.into(),
        }
    }
}
