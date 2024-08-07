use actix_web::{App, HttpServer};

use crate::actix_config;

pub async fn run_actix() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(actix_config::create_config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
