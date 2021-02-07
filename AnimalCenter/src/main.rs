mod animal;
mod controller;
mod entity;
mod infrastruct;
mod model;
mod repository;
mod route;
mod route_fake;
mod route_graphql;
mod service;

use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use animal::{AnimalSchema, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use opentelemetry::{
    global,
    sdk::{
        export::trace::SpanExporter,
        propagation::TraceContextPropagator,
        trace::{BatchSpanProcessor, TracerProvider},
    },
};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = opentelemetry_jaeger::new_pipeline()
        .with_service_name("animal_center")
        .init_exporter()
        .expect("pipeline install error");
    let tracer_provider = TracerProvider::builder()
        .with_batch_exporter(tokio_exporter_compat(exporter))
        .build();
    let _uninstall = global::set_tracer_provider(tracer_provider);

    HttpServer::new(move || {
        let mut app = App::new();
        app = route_fake::makefakeroute(app);
        app.wrap(RequestTracing::new())
            // #region -> base curd service
            .service(route::bussisscope())
            // #endregion
            // #region -> graphql
            .data(schema.clone())
            .service(route_graphql::graphqlscope())
        // #endregion
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
