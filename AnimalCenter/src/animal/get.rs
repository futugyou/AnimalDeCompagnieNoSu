use crate::animal::animal::Animal;
use actix_web::{web, HttpRequest, HttpResponse};
pub async fn animal_handler(item: web::Json<Animal>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}
