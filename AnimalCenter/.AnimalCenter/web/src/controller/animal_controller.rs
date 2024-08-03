use model::animal::{animalinsertmodel::*, animalsearchmodel::*, animalupdatemodel::*};
use service::animalservice::*;

use actix_web::{
    web::{self, Query},
    Error, HttpRequest, HttpResponse,
};
use std::collections::HashMap;

pub async fn getone(web::Path((id,)): web::Path<(String,)>, _req: HttpRequest) -> HttpResponse {
    let service = AnimalService::new().await;
    let response = service.find_animal_by_id(id).await;
    HttpResponse::Ok().json(response)
}

pub async fn get(Query(request): Query<AnimalSearchRequest>, _req: HttpRequest) -> HttpResponse {
    let service = AnimalService::new().await;
    let response = service.search_animals(request).await;
    HttpResponse::Ok().json(response)
}

pub async fn post(
    item: Option<web::Json<AnimalInsertRequest>>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let service = AnimalService::new().await;
    let mut rep = AnimalInsertRequest::default();
    match item {
        Some(i) => {
            rep = i.into_inner();
        }
        None => {}
    };
    let response = service.insert_animal(rep).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn update(
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
    service.delete_animal(id).await?;
    Ok(HttpResponse::Ok().json("ok"))
}

pub async fn clearfake(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let service = AnimalService::new().await;
    service.clear_fake_data().await?;
    Ok(HttpResponse::Ok().json("ok"))
}
