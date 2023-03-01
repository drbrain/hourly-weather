use crate::activity_pub::{link::Link, CONTEXT};
use activitystreams_kinds::object::ImageType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "@context")]
    context: String,
    r#type: ImageType,
    name: String,
    url: Vec<Link>,
}

impl Image {
    pub fn new(name: impl Into<String>, url: Vec<Link>) -> Self {
        Self {
            context: CONTEXT.into(),
            r#type: ImageType::Image,
            name: name.into(),
            url,
        }
    }
}
