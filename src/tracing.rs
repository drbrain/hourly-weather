pub fn init() {
    axum_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()
        .expect("Failed to initialize tracing");
}
