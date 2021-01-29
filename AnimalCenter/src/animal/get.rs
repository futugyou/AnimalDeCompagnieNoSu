use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};
use crate::infrastruct::date_format_option;
use crate::infrastruct::deserialize_object_id;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
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
    #[serde(alias = "_id", default, deserialize_with = "deserialize_object_id")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub animal_type: String,
    #[serde(default)]
    pub sub_type: String,
    #[serde(with = "date_format_option", default)]
    pub birthday: Option<DateTime<Utc>>,
    #[serde(default)]
    pub idcard: String,
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
        let basn = Bson::Document(result.unwrap());
        println!("{:?}", basn);
        let b = bson::from_bson(basn);
        let animal = b.unwrap();
        animals.push(animal)
    }
    HttpResponse::Ok().json(animals)
}
