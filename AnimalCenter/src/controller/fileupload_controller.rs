use actix_web::HttpRequest;
use rustc_serialize::base64::{ToBase64, MIME};
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use bson::{doc, spec::BinarySubtype, Binary, Bson};
use futures::{StreamExt, TryStreamExt};

use crate::infrastruct::context::dbcontext::{DBContext, IDbContext};

pub async fn post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let dbcontext = DBContext {};
    let dbclient = dbcontext.get_db_context().await.unwrap();
    let collection = dbclient.database("react-app").collection("upload");
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}", sanitize_filename::sanitize(&filename));
        println!("filepath: {:?}", filepath);
        // File::create is blocking operation, use threadpool
        // let mut f = web::block(|| std::fs::File::create(filepath))
        //     .await
        //     .unwrap();
        let mut file_data = Vec::<u8>::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_data.extend_from_slice(&data);
            // filesystem operations are blocking, we have to use threadpool
            //f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
        //web::block(move || f.write_all(&file_data).map(|_| f)).await?;
        let binary = Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: file_data,
        });
        let docs = doc! {
                "name":binary,
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
