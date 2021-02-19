use crate::{
    model::animal::animaltypemodel::AnimalTypeUpdateRequest,
    service::animaltypeservice::{AnimalTypeService, IAnimalTypeService},
};

use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::model::animal::animaltypemodel::AnimalTypeSearchRequest;

pub async fn get(
    item: Option<web::Json<AnimalTypeSearchRequest>>,
    _req: HttpRequest,
) -> HttpResponse {
    let service = AnimalTypeService::new().await;
    let mut rep = AnimalTypeSearchRequest {
        animal_type: Vec::<String>::new(),
    };
    match item {
        Some(i) => {
            let query = i.into_inner();
            rep.animal_type = query.animal_type;
        }
        None => {}
    };
    let response = service.search_animal_types(rep).await;
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
