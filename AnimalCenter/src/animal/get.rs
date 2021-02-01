use crate::{
    infrastruct::context::dbcontext::{DBContext, IDbContext},
    viewmodel::animal::animalviewmodel::{AnimalSearchRequest, AnimalSearchResponse},
};
use actix_web::{web, HttpRequest, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

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
                filter.insert("name", doc! {"$regex": query.name});
            }
            if query.animal_type.len() > 0 {
                filter.insert(
                    "$or",
                    vec![
                        doc! {"type":doc!{"$in":&query.animal_type}},
                        doc! {"sub_type":doc!{"$in":&query.animal_type}},
                    ],
                );
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
        let b = bson::from_bson(basn);
        let animal = b.unwrap();
        animals.push(animal)
    }
    HttpResponse::Ok().json(animals)
}
