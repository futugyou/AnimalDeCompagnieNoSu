use crate::controller;

use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    guard, web, Error, Scope,
};

use infrastruct::config::Config;

pub fn bussisscope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let _config = Config::new();
    let apikey = _config.api_key;
    let apivalue = _config.api_value;

    let cors = Cors::default()
        // .send_wildcard()
        // .allowed_origin("http://localhost:5000")
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    web::scope("/api")
        .wrap(cors)
        .guard(guard::fn_guard(move |req| {
            match req.head().headers.get(apikey.clone()) {
                Some(value) => value == apivalue.as_str(),
                None => false,
            }
        }))
        .configure(reportroute)
        .configure(animalroute)
        .configure(animaleventroute)
        .configure(fileuploadroute)
        .configure(animaltyperoute)
        .configure(staticfileuploadroute)
}

fn animaleventroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal/{id}/event")
            .route(web::get().to(controller::animalevent_controller::get))
            .route(
                web::route()
                    //.guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(controller::animalevent_controller::post),
            ),
    );
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
                    //.guard(guard::Header("Content-Type", "application/json"))
                    .guard(guard::Post())
                    .to(controller::animal_controller::post),
            )
            .route(
                web::route()
                    //.guard(guard::Header("Content-Type", "application/json"))
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

fn staticfileuploadroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/staticfile")
            .route(web::get().to(controller::fileupload_controller::get))
            .route(
                web::route()
                    // multipart/form-data; boundary=<calculated when request is sent>
                    //.guard(guard::Header("Content-Type", "multipart/form-data"))
                    .guard(guard::Post())
                    .to(controller::staticfile_controller::post),
            ),
    );
}

fn reportroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animalreport")
            .route(web::get().to(controller::report_controller::get_rescue_classification)),
    );
}
