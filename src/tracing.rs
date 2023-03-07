use axum_tracing_opentelemetry::{jaeger, resource::DetectResource};
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt};

pub fn init() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var("OTEL_LOG_LEVEL"))
            .unwrap_or("hourly_weather=debug".to_string()),
    );

    let otel_resource = DetectResource::default()
        .with_fallback_service_name("hourly_weather")
        .with_fallback_service_version(env!("CARGO_PKG_VERSION"))
        .build();

    let otel_tracer = jaeger::init_tracer(otel_resource, jaeger::identity)
        .expect("failed to initialize OTEL tracer");

    let otel_layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_timer(tracing_subscriber::fmt::time::uptime());

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .with(otel_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to register global tracing subscriber");
}
