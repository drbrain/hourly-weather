use super::CONTEXT;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Context {
    String(String),
    Json(serde_json::Value),
}

impl Context {
    pub fn activitystreams() -> Self {
        Self::from_str(CONTEXT)
    }

    pub fn from_json_str(json: &str) -> Self {
        Self::Json(serde_json::from_str(json).unwrap())
    }

    pub fn from_str(str: &str) -> Self {
        Self::String(str.into())
    }
}
