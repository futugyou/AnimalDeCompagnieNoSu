use actix_web::{get, post, HttpResponse, Responder};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use crate::route::{crudroute, route_graphql};

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

pub async fn run_actix() -> std::io::Result<()> {
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
