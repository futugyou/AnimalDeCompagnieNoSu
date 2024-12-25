// use opentelemetry::{global, trace::TracerProvider, KeyValue};
// use opentelemetry_otlp::WithExportConfig as _;
// use opentelemetry_sdk::{runtime, trace::Tracer, Resource};
// use std::{io, time::Duration};
// use tonic::metadata::MetadataMap;
// use tracing::level_filters::LevelFilter;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_opentelemetry::OpenTelemetryLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// use infrastruct::config::Config;

// pub fn init() {
//     let app_name = "animal-center";

//     let tracer = opentelemetry_tracer_provider(app_name);

//     // we prefer the bunyan formatting layer in this example because it captures
//     // span enters and exits by default, making a good way to observe request
//     // info like duration when
//     let stdout_log = BunyanFormattingLayer::new(app_name.to_owned(), io::stdout);

//     tracing_subscriber::registry()
//         .with(
//             EnvFilter::builder()
//                 .with_default_directive(LevelFilter::INFO.into())
//                 .from_env_lossy(),
//         )
//         .with(OpenTelemetryLayer::new(tracer))
//         .with(JsonStorageLayer)
//         .with(stdout_log)
//         .init();
// }

// fn opentelemetry_tracer_provider(app_name: &str) -> Tracer {
//     let _config = Config::new();
//     let honeycomb_key = _config.honeycomb_api_key;

//     let mut metadata = MetadataMap::with_capacity(1);
//     metadata.insert("x-honeycomb-team", honeycomb_key.try_into().unwrap());

//     let trace_config =
//         opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
//             KeyValue::new("service.name", app_name.to_owned()),
//         ]));
//     let exporter = opentelemetry_otlp::new_exporter()
//         .tonic()
//         .with_endpoint("https://api.honeycomb.io/api/traces")
//         .with_timeout(Duration::from_secs(5))
//         .with_metadata(metadata);

//     let provider = opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_trace_config(trace_config)
//         .with_exporter(exporter)
//         .install_batch(runtime::TokioCurrentThread)
//         .unwrap();

//     global::set_tracer_provider(provider.clone());
//     provider.tracer(app_name.to_owned())
// }
