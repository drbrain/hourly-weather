use crate::HourlyWeather;
use actix_web::web::Data;
use actix_webfinger::Webfinger;
use std::{future::Future, pin::Pin};
use tracing::trace;

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
        let subject = format!("{}@{}", account, domain);

        trace!("webfinger lookup: {subject}");

        let w = if scheme == Some("acct:") && domain == state.domain && account == "hourly" {
            let webfinger = Webfinger::new(&subject)
                .add_profile("http://weather.segment7.net/hourly")
                .clone();

            Some(webfinger)
        } else {
            None
        };

        Box::pin(async move { Ok(w) })
    }
}
