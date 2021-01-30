use actix_web::web;

use crate::animal;

pub fn animalroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal")
            .route(web::get().to(animal::get::animal_handler))
            .route(web::post().to(animal::post::animal_handler)),
    );
}
