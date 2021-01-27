use crate::animal::animal::Animal;
use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};
use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::bson::doc;

pub async fn animal_handler(item: Option<web::Json<Animal>>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req.path());
    println!("model: {:?}", item);
    match item {
        Some(animal) => {
            let animal = animal.into_inner();
            let dbcontext = DBContext {};
            let dbclient = dbcontext.get_db_context().await;
            let collection = dbclient.database("react-app").collection("animal");
            let docs = doc! { "name": animal.name, "animal_type": animal.animal_type };
            let result = collection.insert_one(docs, None).await.unwrap();
            HttpResponse::Ok().json(result)
        }
        None => HttpResponse::Ok().json("no data"),
    }
}
