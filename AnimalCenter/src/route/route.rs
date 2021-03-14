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
        .configure(reportroute)
        .configure(animalroute)
        .configure(fileuploadroute)
        .configure(animaltyperoute)
}

fn animalroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal/{id}").route(web::get().to(controller::animal_controller::getone)),
    );
    cfg.service(
        web::resource("/animal")
            .route(web::delete().to(controller::animal_controller::delete))
            .route(web::get().to(controller::animal_controller::get))
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(controller::animal_controller::post),
            )
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Put())
                    .to(controller::animal_controller::update),
            ),
    );
    cfg.service(
        web::resource("/animal/clearfakedata")
            .route(web::post().to(controller::animal_controller::clearfake)),
    );
}

fn animaltyperoute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animaltype")
            .route(web::get().to(controller::animaltype_controller::get))
            .route(
                web::route()
                    .guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(controller::animaltype_controller::post),
            ),
    );
}
fn fileuploadroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animalfileupload")
            .route(web::get().to(controller::fileupload_controller::get))
            .route(
                web::route()
                    // multipart/form-data; boundary=<calculated when request is sent>
                    //.guard(guard::Header("Content-Type", "multipart/form-data"))
                    .guard(guard::Post())
                    .to(controller::fileupload_controller::post),
            ),
    );
}

fn reportroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animalreport")
            .route(web::get().to(controller::report_controller::get_rescue_classification)),
    );
}
