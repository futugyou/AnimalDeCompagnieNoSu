use crate::service::animaltypeservice::{AnimalTypeService, IAnimalTypeService};
use model::animaltype::animaltypemodel::*;

use actix_web::{
    web::{self, Query},
    Error, HttpRequest, HttpResponse,
};

pub async fn get(
    Query(request): Query<AnimalTypeSearchRequest>,
    _req: HttpRequest,
) -> HttpResponse {
    let service = AnimalTypeService::new().await;
    let response = service.search_animal_types(request).await;
    HttpResponse::Ok().json(response)
}

pub async fn post(
    item: Option<web::Json<AnimalTypeUpdateRequest>>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let service = AnimalTypeService::new().await;
    let mut rep = AnimalTypeUpdateRequest::new();
    match item {
        Some(i) => {
            rep = i.into_inner();
        }
        None => {}
    };
    let response = service.modfiy_animal_type(rep).await;
    Ok(HttpResponse::Ok().json(response))
}
