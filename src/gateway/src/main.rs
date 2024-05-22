use anyhow::Context;
use config::{Config, Environment};
use gateway::AppConfig;
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self as sdktrace, TracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let env_file = std::env::var("ENV_FILE").unwrap_or_else(|_| "env.json".to_string());

    let config: AppConfig = Config::builder()
        .add_source(config::File::with_name(&env_file))
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("config build")?
        .try_deserialize()
        .context("config deserialize")?;

    init_telemtry();
    gateway::start(config).await
}

fn init_telemtry() {
    const APP_NAME: &str = "e-transfer";
    const APP_TELEMETRY_ENDPOINT: &str = "http://localhost:14268/api/traces";

    let tracer = if false {
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(APP_TELEMETRY_ENDPOINT),
            )
            .with_trace_config(
                sdktrace::config()
                    .with_resource(Resource::new(vec![KeyValue::new(SERVICE_NAME, APP_NAME)])),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap()
    } else {
        TracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build()
            .tracer(APP_NAME)
    };

    // initialize `tracing` using `opentelemetry-tracing` and configure logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("INFO"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    //let formatting_layer = tracing_subscriber::fmt::layer();
    let formatting_layer = BunyanFormattingLayer::new(APP_NAME.into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initialise `tracing` subscriber.")
}
