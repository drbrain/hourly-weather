use crate::HourlyWeather;
use axum::{
    async_trait,
    extract::{FromRequestParts, Query, State},
    http::request::Parts,
};
use hyper::StatusCode;
use std::{collections::HashMap, sync::Arc};
use webfinger::{Link, Webfinger};

pub struct ExtractResource(Account);

struct Account {
    account: String,
}

impl Account {
    fn try_new(resource: &str) -> Option<Self> {
        let parts: Vec<_> = resource.splitn(2, ':').collect();

        if parts.len() != 2 {
            return None;
        }

        if parts[0] != "acct" {
            return None;
        }

        let account = parts[1];

        if account.splitn(2, '@').count() != 2 {
            return None;
        }

        Some(Self {
            account: account.to_string(),
        })
    }

    fn domain(&self) -> &str {
        self.account.splitn(2, '@').last().unwrap()
    }

    fn user(&self) -> &str {
        self.account.splitn(2, '@').next().unwrap()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractResource
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Ok(query): Result<Query<HashMap<String, String>>, _> = Query::from_request_parts(parts, state).await else {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, "resource missing"));
        };

        let Some(resource) = query.get("resource") else {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, "resource missing"));
        };

        let Some(account) = Account::try_new(resource) else {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, "invalid or unsupported resource"));
        };

        Ok(ExtractResource(account))
    }
}

pub async fn webfinger(
    State(state): State<Arc<HourlyWeather>>,
    ExtractResource(account): ExtractResource,
) -> Result<String, StatusCode> {
    if state.domain() != account.domain() {
        return Err(StatusCode::NOT_FOUND);
    } else if account.user() != "hourly" {
        return Err(StatusCode::NOT_FOUND);
    } else {
        let domain = state.domain();

        let links = vec![
            Link {
                rel: "self".into(),
                href: Some(format!("https://{domain}/hourly")),
                mime_type: Some("application/activity+json".into()),
                template: None,
            },
            Link {
                rel: "http://webfinger.net/rel/profile-page".into(),
                href: Some(format!("https://{domain}/hourly/about")),
                mime_type: Some("text/html".into()),
                template: None,
            },
        ];

        let webfinger = Webfinger {
            subject: format!("acct:hourly@{domain}"),
            links,
            aliases: vec![
                format!("https://{domain}/hourly"),
                format!("https://{domain}/hourly/about"),
            ],
        };

        serde_json::to_string(&webfinger).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
