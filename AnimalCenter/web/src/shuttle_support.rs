use actix_web::web::ServiceConfig;
use anyhow::Context;
use shuttle_actix_web::ShuttleActixWeb;

use crate::actix_config;

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
    let honeycomb_api_key = secrets.get("honeycomb_api_key").context("secret was not found")?;

    std::env::set_var("mongodb_uri", mongodb_uri);
    std::env::set_var("table_name", table_name);
    std::env::set_var("api_key", api_key);
    std::env::set_var("api_value", api_value);
    std::env::set_var("server_address", server_address);
    std::env::set_var("amqp_addr", amqp_addr);
    std::env::set_var("honeycomb_api_key", honeycomb_api_key);

    let config = move |cfg: &mut ServiceConfig| {
        actix_config::create_config(cfg);
    };

    Ok(config.into())
}
