use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse, Responder};
use actix_web_lab::{extract::ThinData, middleware::from_fn};
use dotenv::dotenv;
use metrics_exporter_prometheus::PrometheusHandle;

use crate::observable;
use crate::route::{crudroute, route_graphql};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}

pub fn create_config(cfg: &mut ServiceConfig) {
    dotenv().ok();

    observable::logging::init();
    let handle = observable::prometheus::init();

    cfg.app_data(ThinData(handle.clone()));

    cfg.route("/healthz", actix_web::web::get().to(health_check));
    cfg.service(metrics);

    cfg.service(crudroute::bussisscope().wrap(from_fn(observable::middleware::request_telemetry)));
    cfg.service(route_graphql::graphqlscope());
}

#[get("/metrics")]
pub(crate) async fn metrics(metrics_handle: ThinData<PrometheusHandle>) -> impl Responder {
    metrics_handle.render()
}
