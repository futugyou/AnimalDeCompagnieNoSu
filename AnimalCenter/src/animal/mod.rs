use actix_web::web;
mod animal;
mod get;
mod post;
pub fn animalconfig(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/animal")
            .route(web::get().to(get::animal_handler))
            .route(web::post().to(post::animal_handler)),
    );
}
