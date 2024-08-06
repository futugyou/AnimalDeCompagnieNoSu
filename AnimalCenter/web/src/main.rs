mod controller;
mod graphql;
mod route;

#[cfg(not(feature = "actix"))]
mod shuttle_support;

#[cfg(feature = "actix")]
mod actix_support;

#[cfg(not(feature = "actix"))]
#[shuttle_runtime::main]
async fn main() -> shuttle_actix_web::ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static>  {
    shuttle_support::run_shuttle().await
}

#[cfg(feature = "actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_support::run_actix().await
}
