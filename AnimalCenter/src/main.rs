mod animal;
mod infrastruct;
mod route;
mod route_fake;
mod route_graphql;

use actix_web::{guard, web, App, HttpServer};
use animal::{AnimalSchema, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        let app = App::new();
        let app = route_fake::makefakeroute(app);
        app
            // #region -> base curd service
            .service(route::bussisscope())
            // #endregion
            // #region -> graphql
            .data(schema.clone())
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(route_graphql::index_playground),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(route_graphql::graphql_index),
            )
        // #endregion
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
