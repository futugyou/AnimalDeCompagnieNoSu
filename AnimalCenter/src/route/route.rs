use crate::controller;
use crate::infrastruct::config::Config;
use actix_web::{guard, web, Scope};

pub fn bussisscope() -> Scope {
    let _config = Config::new();
    let apikey = _config.api_key;
    let apivalue = _config.api_value;
    web::scope("/api")
        .guard(guard::fn_guard(move |req| {
            match req.headers().get(apikey.clone()) {
                Some(value) => value == apivalue.as_str(),
                None => false,
            }
        }))
        .configure(animalroute)
        .configure(animaltyperoute)
}

fn animalroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal")
            .route(web::delete().to(controller::animal_controller::delete))
            .route(web::get().to(controller::animal_controller::get))
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    //.to(animal::post::animal_handler),
                    .to(controller::animal_controller::post),
            ),
    );
}

fn animaltyperoute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animaltype")
            .route(web::get().to(|| actix_web::HttpResponse::Ok()))
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(|| actix_web::HttpResponse::Ok()),
            ),
    );
}
