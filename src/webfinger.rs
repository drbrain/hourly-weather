use crate::HourlyWeather;
use actix_web::web::Data;
use actix_webfinger::Webfinger;
use std::{future::Future, pin::Pin};

type LocalBoxFuture<'a, Output> = Pin<Box<dyn Future<Output = Output> + 'a>>;

pub struct Resolver;

impl actix_webfinger::Resolver for Resolver {
    type State = Data<HourlyWeather>;
    type Error = actix_web::error::JsonPayloadError;

    fn find(
        scheme: Option<&str>,
        account: &str,
        domain: &str,
        state: Data<HourlyWeather>,
    ) -> LocalBoxFuture<'static, Result<Option<Webfinger>, Self::Error>> {
        let w = if scheme == Some("acct:") && domain == state.domain && account == "hourly" {
            Some(Webfinger::new(&format!("{}@{}", account, domain)))
        } else {
            None
        };

        Box::pin(async move { Ok(w) })
    }
}
