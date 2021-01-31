use crate::animal;
use crate::infrastruct::config::{Config, IConfig};
use actix_web::{guard, web, Scope};

pub fn bussisscope() -> Scope {
    let _config = Config {};
    let apikey = _config.get_config_with_key("API_KEY");
    let apivalue = _config.get_config_with_key("API_VALUE");
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
            .route(web::get().to(animal::get::animal_handler))
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(animal::post::animal_handler),
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
