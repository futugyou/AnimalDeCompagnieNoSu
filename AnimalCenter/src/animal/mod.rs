mod animal;
mod get;
mod post;

use actix_web::web;

pub use animal::{AnimalSchema, QueryRoot};

pub fn animalconfig(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal")
            .route(web::get().to(get::animal_handler))
            .route(web::post().to(post::animal_handler)),
    );
}
