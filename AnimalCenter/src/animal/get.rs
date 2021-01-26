use crate::animal::animal::Animal;
use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

pub async fn animal_handler(item: Option<web::Json<Animal>>, req: HttpRequest) -> HttpResponse {
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
                filter.insert("animal_type", query.animal_type);
            }
        }
        None => {}
    };
    let collection = dbclient.database("react-app").collection("animal");
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut animals = Vec::<Animal>::new();
    while let Some(result) = cursor.next().await {
        let animal: Animal = bson::from_bson(Bson::Document(result.unwrap())).unwrap();
        animals.push(animal)
    }
    HttpResponse::Ok().json(animals)
}
