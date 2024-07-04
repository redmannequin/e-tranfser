use anyhow::Context;
use config::{Config, Environment};
use payment_manager::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig = Config::builder()
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("config build")?
        .try_deserialize()
        .context("config deserialize")?;

    // init_tracing(&config.otel_config);
    payment_manager::start(config).await
}

// fn init_tracing(config: &OtelConfig) {
//     let tracer = create_otlp_tracer(config).unwrap();
//     let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("INFO"));
//     let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
//     //let formatting_layer = tracing_subscriber::fmt::layer();
//     let formatting_layer = BunyanFormattingLayer::new(config.service_name.clone(), std::io::stdout);

//     let subscriber = Registry::default()
//         .with(env_filter)
//         .with(telemetry)
//         .with(JsonStorageLayer)
//         .with(formatting_layer);

//     opentelemetry::global::set_text_map_propagator(
//         opentelemetry_sdk::propagation::TraceContextPropagator::default(),
//     );

//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Failed to initialise `tracing` subscriber.");
// }

// fn create_otlp_tracer(config: &OtelConfig) -> Option<opentelemetry_sdk::trace::Tracer> {
//     let protocol = &config.exporter_otlp_protocol;
//     let tracer = opentelemetry_otlp::new_pipeline().tracing();
//     let headers = &config.exporter_otlp_headers;
//     let endpoint = &config.exporter_otlp_endpoint;

//     std::env::set_var("OTEL_SERVICE_NAME", &config.service_name);
//     std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", endpoint);
//     std::env::set_var(
//         "OTEL_EXPORTER_OTLP_HEADERS",
//         headers
//             .iter()
//             .map(|t| t.split_at(t.find(":").unwrap()))
//             .map(|t| format!("{}={}", t.0, t.1))
//             .collect::<Vec<_>>()
//             .join(","),
//     );
//     std::env::set_var("OTEL_EXPORTER_OTLP_PROTOCOL", protocol);

//     let tracer =
//         match protocol.as_str() {
//             "grpc" => {
//                 let mut exporter = opentelemetry_otlp::new_exporter().tonic().with_metadata(
//                     metadata_from_headers(headers.iter().map(|t| t.split_at(t.find('=').unwrap()))),
//                 );
//                 // Check if we need TLS
//                 if endpoint.starts_with("https") {
//                     exporter = exporter.with_tls_config(Default::default());
//                 }
//                 tracer.with_exporter(exporter)
//             }
//             "http/protobuf" => {
//                 let exporter = opentelemetry_otlp::new_exporter().http().with_headers(
//                     headers
//                         .iter()
//                         .map(|t| {
//                             let a = t.split_at(t.find('=').unwrap());
//                             (a.0.to_owned(), a.1.to_owned())
//                         })
//                         .collect(),
//                 );
//                 tracer.with_exporter(exporter)
//             }
//             p => panic!("Unsupported protocol {}", p),
//         };

//     Some(
//         tracer
//             .install_batch(opentelemetry_sdk::runtime::Tokio)
//             .unwrap(),
//     )
// }

// fn metadata_from_headers<'a>(
//     headers: impl Iterator<Item = (&'a str, &'a str)>,
// ) -> tonic::metadata::MetadataMap {
//     use tonic::metadata;
//     let mut metadata = metadata::MetadataMap::new();
//     headers.for_each(|(name, value)| {
//         let value = value
//             .parse::<metadata::MetadataValue<metadata::Ascii>>()
//             .expect("Header value invalid");
//         metadata.insert(metadata::MetadataKey::from_str(name).unwrap(), value);
//     });
//     metadata
// }
