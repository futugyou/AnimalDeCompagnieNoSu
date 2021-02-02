mod animal;
mod infrastruct;
mod model;
mod repository;
mod route;
mod route_fake;
mod route_graphql;
mod service;

use actix_web::{App, HttpServer};
use animal::{AnimalSchema, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        let mut app = App::new();
        app = route_fake::makefakeroute(app);
        app
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
