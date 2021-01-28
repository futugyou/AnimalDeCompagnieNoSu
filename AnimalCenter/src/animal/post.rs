use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};
use crate::infrastruct::date_format;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
    #[serde(default)]
    pub sub_type: String,
    #[serde(with = "date_format")]
    pub birthday: DateTime<Utc>,
}
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
            let docs = doc! { "name": animal.name, "animal_type": animal.animal_type };
            let result = collection.insert_one(docs, None).await.unwrap();
            HttpResponse::Ok().json(result)
        }
        None => HttpResponse::Ok().json("no data"),
    }
}
