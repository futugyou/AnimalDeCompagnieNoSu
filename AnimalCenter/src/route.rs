use crate::animal;
use crate::infrastruct::config::{Config, IConfig};
use actix_web::{guard, web};

pub fn animalroute(cfg: &mut web::ServiceConfig) {
    let _config = Config {};
    let apikey = _config.get_config_with_key("API_KEY");
    let apivalue = _config.get_config_with_key("API_VALUE");
    cfg.service(
        web::resource("/animal")
            .guard(guard::fn_guard(move |req| {
                match req.headers().get(apikey.clone()) {
                    Some(value) => value == apivalue.as_str(),
                    None => false,
                }
            }))
            .route(web::get().to(animal::get::animal_handler))
            .route(web::post().to(animal::post::animal_handler)),
    );
}
