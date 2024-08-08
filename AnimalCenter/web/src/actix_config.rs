use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use dotenv::dotenv;

use crate::observable;
use crate::route::{crudroute, route_graphql};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}

pub fn create_config(cfg: &mut ServiceConfig) {
    dotenv().ok();

    observable::logging::init();

    cfg.route("/healthz", actix_web::web::get().to(health_check));
    cfg.service(crudroute::bussisscope().wrap(from_fn(observable::middleware::request_telemetry)));
    cfg.service(route_graphql::graphqlscope());
}
