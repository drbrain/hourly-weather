use crate::activity_pub::link::Link;
use activitystreams_kinds::object::ImageType;
use serde::Serialize;

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
