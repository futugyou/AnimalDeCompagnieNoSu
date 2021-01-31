mod animal;
mod infrastruct;
mod route;
mod route_fake;
mod route_graphql;

use crate::route_fake::*;

use actix_web::{guard, web, App, HttpServer};
use animal::{AnimalSchema, QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_counter = web::Data::new(AppStatwWithCounter::new());
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        App::new()
            // #region -> base usage
            .service(echo)
            // #endregion
            // #region -> data and app_data
            .data(AppState {
                app_name: String::from("hello world"),
            })
            .app_data(app_counter.clone())
            .service(index)
            .service(hello)
            // #endregion
            // #region -> use config
            .configure(configtest)
            // #endregion
            // #region -> web::scope
            .service(
                web::scope("/guard")
                    .guard(guard::Header("Host", "www.rust-lang.org")) //that means Header must had Host, and value is www.rust-lang.org
                    .route("", web::get().to(manual_hello)),
            )
            .service(web::scope("/app").route("/hello", web::get().to(manual_hello)))
            // #endregion
            // #region -> direct route
            .route("/hey", web::get().to(manual_hello))
            // #endregion
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
