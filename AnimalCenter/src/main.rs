mod controller;
mod entity;
mod graphql;
mod infrastruct;
mod model;
mod repository;
mod route;
mod service;
mod telemetry;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use async_graphql::Schema;
use dotenv::dotenv;
use graphql::*;
use route::{route as orgroute, route_fake, route_graphql};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let metrics = telemetry::initmetrics();
    let _uninstall = telemetry::inittracer();
    let schema = Schema::new(QueryRoot::default(), MutationRoot {}, SubscriptionRoot {});
    HttpServer::new(move || {
        let mut app = App::new();
        app = route_fake::makefakeroute(app);
        app.wrap(Logger::default())
            .wrap(RequestTracing::new())
            .wrap(metrics.clone())
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
