use actix_web::web::ServiceConfig;
use actix_web::{get, post, HttpResponse, Responder};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;

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

pub async fn run_shuttle(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
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
