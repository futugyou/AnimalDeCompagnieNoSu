use crate::model::animal::animalmodel::{AnimalSearchRequest, AnimalUpdateRequest};

use actix_web::{web, HttpRequest, HttpResponse};

pub async fn get(item: Option<web::Json<AnimalSearchRequest>>, req: HttpRequest) -> HttpResponse {
    todo!()
}

pub async fn post(item: Option<web::Json<AnimalUpdateRequest>>, req: HttpRequest) -> HttpResponse {
    todo!()
}
