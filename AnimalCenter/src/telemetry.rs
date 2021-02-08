use opentelemetry::{
    global::{self, TracerProviderGuard},
    sdk::{
        export::trace::SpanExporter,
        propagation::TraceContextPropagator,
        trace::{BatchSpanProcessor, TracerProvider as tr},
    },
    trace::TracerProvider,
};
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

pub fn init() -> TracerProviderGuard {
    // Start an otel jaeger trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = opentelemetry_jaeger::new_pipeline()
        .with_service_name("animal_center")
        .init_exporter()
        .expect("pipeline install error");
    let tracer_provider = tr::builder()
        .with_batch_exporter(tokio_exporter_compat(exporter))
        .build();
    let tracer = tracer_provider.get_tracer("", None);
    // Initialize `tracing` using `opentelemetry-tracing` and configure logging
    Registry::default()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
    global::set_tracer_provider(tracer_provider)
}

// Compatibility with older tokio v0.2.x used by actix web v3. Not necessary with actix web v4.
fn tokio_exporter_compat<T: SpanExporter + 'static>(exporter: T) -> BatchSpanProcessor {
    let spawn = |fut| tokio::task::spawn_blocking(|| futures::executor::block_on(fut));
    BatchSpanProcessor::builder(
        exporter,
        spawn,
        tokio::time::delay_for,
        tokio::time::interval,
    )
    .build()
}
