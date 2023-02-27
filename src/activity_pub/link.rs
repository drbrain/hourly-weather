use activitystreams_kinds::link::LinkType;
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
