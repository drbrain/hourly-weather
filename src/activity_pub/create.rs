use crate::activity_pub::image::Image;
use activitystreams_kinds::activity::CreateType;
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
