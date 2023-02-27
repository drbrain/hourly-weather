use std::sync::Arc;

#[derive(Debug)]
pub struct HourlyWeather {
    domain: String,
}

impl HourlyWeather {
    pub fn new(domain: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            domain: domain.into(),
        })
    }

    pub fn actor(&self) -> String {
        format!("https://{}/hourly", self.domain)
    }

    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    pub fn image(&self, date: &str, time: &str) -> String {
        format!("https://{}/images/{date}/{date}-{time}.jpeg", self.domain)
    }

    pub fn outbox(&self) -> String {
        format!("https://{}/hourly/outbox", self.domain)
    }
}
