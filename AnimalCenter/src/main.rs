mod animal;
mod controller;
mod entity;
mod infrastruct;
mod model;
mod repository;
mod route;
mod service;
mod telemetry;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use animal::{AnimalSchema, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use route::{route as orgroute, route_fake, route_graphql};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _uninstall = telemetry::init();
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        let mut app = App::new();
        app = route_fake::makefakeroute(app);
        app.wrap(Logger::default())
            .wrap(RequestTracing::new())
            // #region -> base curd service
            .service(orgroute::bussisscope())
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
