use actix_web::web::ServiceConfig;
use actix_web::{get, post, HttpResponse, Responder};
use anyhow::Context;
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
    secrets: shuttle_runtime::SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let mongodb_uri = secrets.get("mongodb_uri").context("secret was not found")?;
    let table_name = secrets.get("table_name").context("secret was not found")?;
    let api_key = secrets.get("api_key").context("secret was not found")?;
    let api_value = secrets.get("api_value").context("secret was not found")?;
    let server_address = secrets
        .get("server_address")
        .context("secret was not found")?;
    let amqp_addr = secrets.get("amqp_addr").context("secret was not found")?;
    let file_upload_path = secrets
        .get("file_upload_path")
        .context("secret was not found")?;

    std::env::set_var("mongodb_uri", mongodb_uri);
    std::env::set_var("table_name", table_name);
    std::env::set_var("api_key", api_key);
    std::env::set_var("api_value", api_value);
    std::env::set_var("server_address", server_address);
    std::env::set_var("amqp_addr", amqp_addr);
    std::env::set_var("file_upload_path", file_upload_path);

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
