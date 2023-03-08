use std::path::Path;
use tower::ServiceBuilder;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    services::ServeFile,
    trace::{Trace, TraceLayer},
};

pub fn sky_jpeg<P>(sky_jpeg: P) -> Trace<ServeFile, SharedClassifier<ServerErrorsAsFailures>>
where
    P: AsRef<Path>,
{
    ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .service(ServeFile::new(sky_jpeg))
}
