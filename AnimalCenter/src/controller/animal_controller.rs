use crate::model::animal::animalmodel::{AnimalSearchRequest, AnimalUpdateRequest};
use crate::service::animalservice::{AnimalService, IAnimalService};

use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::collections::HashMap;

pub async fn get(item: Option<web::Json<AnimalSearchRequest>>, _req: HttpRequest) -> HttpResponse {
    let service = AnimalService::new().await;
    let mut rep = AnimalSearchRequest {
        name: String::from(""),
        animal_type: Vec::<String>::new(),
    };
    match item {
        Some(i) => {
            let query = i.into_inner();
            rep.name = query.name;
            rep.animal_type = query.animal_type;
        }
        None => {}
    };
    let response = service.search_animals(rep).await;
    HttpResponse::Ok().json(response)
}

pub async fn post(
    item: Option<web::Json<AnimalUpdateRequest>>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let service = AnimalService::new().await;
    let mut rep = AnimalUpdateRequest::new();
    match item {
        Some(i) => {
            rep = i.into_inner();
        }
        None => {}
    };
    let response = service.modfiy_animal(rep).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete(
    parameters: web::Query<HashMap<String, String>>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let id = parameters.get("id").unwrap().to_owned();
    let service = AnimalService::new().await;
    service.delete_animal(id).await;
    Ok(HttpResponse::Ok().json("ok"))
}
