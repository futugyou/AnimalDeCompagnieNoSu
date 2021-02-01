use crate::{
    infrastruct::context::dbcontext::{DBContext, IDbContext},
    viewmodel::animal::animalviewmodel::AnimalUpdateRequest,
};

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use rand::Rng;

pub async fn animal_handler(
    item: Option<web::Json<AnimalUpdateRequest>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("request: {:?}", req.path());
    println!("model: {:?}", item);
    match item {
        Some(animal) => {
            let animal = animal.into_inner();
            let dbcontext = DBContext {};
            let dbclient = dbcontext.get_db_context().await;
            let collection = dbclient.database("react-app").collection("animal");
            let docs = doc! {
                    "name": animal.name,
                    "type": &animal.animal_type,
                    "birthday":animal.birthday,
                    "sub_type":animal.sub_type,
                    "idcard":format!("{}-{}-{:>04}",&animal.animal_type,Utc::now().format("%Y%m%d-%H%M%S"),rand::thread_rng().gen_range(0001..9999)),
            };
            let result = collection.insert_one(docs, None).await.unwrap();
            HttpResponse::Ok().json(result)
        }
        None => HttpResponse::Ok().json("no data"),
    }
}
