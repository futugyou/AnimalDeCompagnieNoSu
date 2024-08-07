use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, Responder};
use dotenv::dotenv;

use crate::route::{crudroute, route_graphql};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}

pub fn create_config(cfg: &mut ServiceConfig) {
    dotenv().ok();

    cfg.route("/healthz", actix_web::web::get().to(health_check));
    cfg.service(crudroute::bussisscope());
    cfg.service(route_graphql::graphqlscope());
}
