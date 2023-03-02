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
        format!("https://{}/hourly/about", self.domain)
    }

    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    pub fn featured(&self) -> String {
        format!("https://{}/hourly/featured", self.domain)
    }

    pub fn following(&self) -> String {
        format!("https://{}/hourly/following", self.domain)
    }

    pub fn followers(&self) -> String {
        format!("https://{}/hourly/followers", self.domain)
    }

    pub fn image(&self, date: &str, time: &str) -> String {
        format!("https://{}/images/{date}/{date}-{time}.jpeg", self.domain)
    }

    pub fn inbox(&self) -> String {
        format!("https://{}/hourly/inbox", self.domain)
    }

    pub fn outbox(&self) -> String {
        format!("https://{}/hourly", self.domain)
    }

    pub fn sky_jpeg(&self) -> String {
        format!("https://{}/sky.jpeg", self.domain)
    }

    pub fn tags(&self) -> String {
        format!("https://{}/hourly/tags", self.domain)
    }
}
