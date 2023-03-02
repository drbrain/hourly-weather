use std::{sync::Arc, time::SystemTime};

use crate::{
    activity_pub::{image::Image, Context},
    HourlyWeather,
};
use activitystreams_kinds::actor::ServiceType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "@context")]
    context: Context,
    r#type: ServiceType,
    discoverable: bool,
    published: SystemTime,
    following: String,
    followers: String,
    featured: String,
    manually_approves_followers: bool,
    tags: String,
    summary: String,
    icon: Image,
    id: String,
    inbox: String,
    name: String,
    outbox: String,
    preferred_username: String,
    url: String,
}

impl Service {
    pub fn new(
        state: Arc<HourlyWeather>,
        name: impl Into<String>,
        icon: Image,
        username: impl Into<String>,
    ) -> Self {
        Self {
            context: Self::context(),
            r#type: ServiceType::Service,
            id: state.actor(),
            name: name.into(),
            icon,
            outbox: state.outbox(),
            inbox: state.inbox(),
            preferred_username: username.into(),
            discoverable: true,
            published: SystemTime::now(),
            following: state.following(),
            followers: state.followers(),
            featured: state.featured(),
            manually_approves_followers: false,
            tags: state.tags(),
            summary: "Hourly Weather from Seattle".into(),
            url: state.actor(),
        }
    }

    fn context() -> Context {
        let context = r#"[
  "https://www.w3.org/ns/activitystreams",
  "https://w3id.org/security/v1",
  {
    "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
    "toot": "http://joinmastodon.org/ns#",
    "featured":
    {
      "@id": "toot:featured",
      "@type": "@id"
    },
    "featuredTags":
    {
      "@id": "toot:featuredTags",
      "@type": "@id"
    },
    "schema": "http://schema.org#",
    "PropertyValue": "schema:PropertyValue",
    "value": "schema:value",
    "discoverable": "toot:discoverable"
  }
]"#;
        Context::from_json_str(context)
    }
}
