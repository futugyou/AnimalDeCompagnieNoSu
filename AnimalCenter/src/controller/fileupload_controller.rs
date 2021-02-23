use actix_web::HttpRequest;
use rustc_serialize::base64::{ToBase64, MIME};

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use bson::doc;
use futures::{StreamExt, TryStreamExt};

use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};

pub async fn post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let dbcontext = DBContext {};
    let dbclient = dbcontext.get_db_context().await.unwrap();
    let collection = dbclient.database("react-app").collection("upload2");
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}", sanitize_filename::sanitize(&filename));
        let ext = filepath.split(".").last().unwrap().to_owned();

        let mut file_data = Vec::<u8>::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_data.extend_from_slice(&data);
        }
        let base64 = file_data.to_base64(MIME);
        let res_base64 = format!("data:image/{};base64,{}", &ext, base64.replace("\r\n", ""));
        let docs = doc! {
                "base64src":res_base64,
        };
        let rr = collection.insert_one(docs, None).await.unwrap();
        println!("----{:#?}", rr);
    }

    Ok(HttpResponse::Ok().into())
}
pub async fn get(_req: HttpRequest) -> HttpResponse {
    let dbcontext = DBContext {};
    let dbclient = dbcontext.get_db_context().await.unwrap();
    let collection = dbclient.database("react-app").collection("upload");
    let mut cursor = collection.find(doc! {}, None).await.unwrap();
    let mut lists = Vec::new();
    while let Some(result) = cursor.next().await {
        let doc = result.unwrap();
        let vec = doc.get_binary_generic("name").unwrap();
        let base64 = vec.to_base64(MIME);
        let res_base64 = format!("data:image/{};base64,{}", "png", base64.replace("\r\n", ""));

        lists.push(res_base64);
    }
    HttpResponse::Ok().json(lists)
}
