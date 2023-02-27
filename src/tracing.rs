use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_rfc_5424::{
//     layer::Layer,
//     rfc5424::Rfc5424,
//     tracing::TrivialTracingFormatter,
//     transport::{Transport, UdpTransport},
// };
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init(app_name: &str) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);

    // FIX: Add to layer
    // let syslog_transport = UdpTransport::new("localhost:9514");
    // let syslog_layer = Layer::<
    //     Layer<tracing_subscriber::Registry, Rfc5424, TrivialTracingFormatter, UdpTransport>,
    // >::with_transport(syslog_transport);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // .with(syslog_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}
