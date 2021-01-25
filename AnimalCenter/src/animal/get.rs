use crate::animal::animal::Animal;
use actix_web::{web, HttpRequest, HttpResponse};
pub async fn animal_handler(item: Option<web::Json<Animal>>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);
    match item {
        None => HttpResponse::Ok().json("ok"),
        Some(i) => HttpResponse::Ok().json(i.0),
    }
}
