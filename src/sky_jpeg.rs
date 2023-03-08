use std::path::Path;
use hyper::Request;
use tower::ServiceBuilder;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    services::ServeFile,
    trace::{MakeSpan, Trace, TraceLayer},
};
use tracing::{Level, Span};

#[derive(Clone)]
pub struct MakeSkyJpegSpan {
    level: Level,
}

impl MakeSkyJpegSpan {
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
        }
    }
}

impl Default for MakeSkyJpegSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeSpan<B> for MakeSkyJpegSpan {
    fn make_span(&mut self, _request: &Request<B>) -> Span {
        macro_rules! span {
            ($level:expr) => {
                tracing::span!($level, "sky_jpeg")
            }
        }

        match self.level {
            Level::ERROR => {
                span!(Level::ERROR)
            },
            Level::WARN => {
                span!(Level::WARN)
            },
            Level::INFO => {
                span!(Level::INFO)
            },
            Level::DEBUG => {
                span!(Level::DEBUG)
            },
            Level::TRACE => {
                span!(Level::TRACE)
            },
        }
    }
}

pub fn sky_jpeg<P>(sky_jpeg: P) -> Trace<ServeFile, SharedClassifier<ServerErrorsAsFailures>, MakeSkyJpegSpan>
where
    P: AsRef<Path>,
{
    let layer = TraceLayer::new_for_http().make_span_with(MakeSkyJpegSpan::default());

    ServiceBuilder::new()
        .layer(layer)
        .service(ServeFile::new(sky_jpeg))
}
