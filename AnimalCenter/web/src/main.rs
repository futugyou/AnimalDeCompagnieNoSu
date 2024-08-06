mod controller;
mod graphql;
mod route;

#[cfg(feature = "actix")]
use actix_web::{App, HttpServer};

#[cfg(not(feature = "actix"))]
use actix_web::web::ServiceConfig;

#[cfg(not(feature = "actix"))]
use shuttle_actix_web::ShuttleActixWeb;

use actix_web::{get, post, HttpResponse, Responder};
use dotenv::dotenv;

use route::{crudroute, route_graphql};

use tool::*;

#[get("/")]
async fn hello() -> impl Responder {
    let datetime = getdefaultdatetime();
    HttpResponse::Ok().body(datetime.to_rfc3339())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(feature = "actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(crudroute::bussisscope())
            .service(route_graphql::graphqlscope())
            .route("/hey", actix_web::web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(not(feature = "actix"))]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();
    let config = move |cfg: &mut ServiceConfig| {
        // set up your service here, e.g.:
        cfg.service(hello);
        cfg.service(echo);
        cfg.service(crudroute::bussisscope());
        cfg.service(route_graphql::graphqlscope());
        cfg.route("/hey", actix_web::web::get().to(manual_hello));
    };

    Ok(config.into())
}
