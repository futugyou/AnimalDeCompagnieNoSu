use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimalSearchResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
}

pub async fn animal_handler(
    item: Option<web::Json<AnimalSearchRequest>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("request: {:?}", req.path());
    println!("model: {:?}", item);
    let dbcontext = DBContext {};
    let dbclient = dbcontext.get_db_context().await;
    let mut filter = doc! {};
    match item {
        Some(i) => {
            let query = i.into_inner();
            if query.name != "" {
                filter.insert("name", query.name);
            }
            if query.animal_type != "" {
                filter.insert("type", query.animal_type);
            }
        }
        None => {}
    };
    let collection = dbclient.database("react-app").collection("animal");
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut animals = Vec::<AnimalSearchResponse>::new();
    while let Some(result) = cursor.next().await {
        let animal = bson::from_bson(Bson::Document(result.unwrap())).unwrap();
        animals.push(animal)
    }
    HttpResponse::Ok().json(animals)
}
