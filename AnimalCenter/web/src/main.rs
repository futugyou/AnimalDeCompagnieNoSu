mod controller;
mod graphql;
mod route;
mod telemetry;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use async_graphql::Schema;
use dotenv::dotenv;
use graphql::*;
use route::{route as orgroute, route_fake, route_graphql};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // telemetry
    let metrics = telemetry::initmetrics();
    let _uninstall = telemetry::inittracer();
    let schema = Schema::new(QueryRoot {}, MutationRoot {}, SubscriptionRoot {});

    // server
    HttpServer::new(move || {
        let mut app = App::new();
        app = route_fake::makefakeroute(app);
        app.wrap(
            Cors::default()
                // .send_wildcard()
                // .allowed_origin("http://localhost:5000")
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600),
        )
        // static file
        .service(Files::new("/images", "fileupload/"))
        .wrap(Logger::default())
        .wrap(RequestTracing::new())
        .wrap(metrics.clone())
        // #region -> base curd service
        .service(orgroute::bussisscope())
        // #endregion
        // #region -> graphql
        .data(schema.clone())
        .service(route_graphql::graphqlscope())
        // #endregion
    })
    // .bind("127.0.0.1:8080")?
    .bind_rustls("127.0.0.1:8080", config)?
    .run()
    .await
}
