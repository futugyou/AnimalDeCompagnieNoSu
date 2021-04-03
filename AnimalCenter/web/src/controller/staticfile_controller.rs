use crate::service::staticfileservice::IStaticfileService;
use crate::service::staticfileservice::StaticfileService;
use model::file::staticfilemodel::*;
use tool::custom_error::CustomError;

use actix_multipart::Multipart;
use actix_web::{web::Bytes, HttpResponse};
use futures::{StreamExt, TryStreamExt};

pub async fn post(mut payload: Multipart) -> Result<HttpResponse, CustomError> {
    let staticserver = StaticfileService {};
    let mut response = StaticfileResponse::default();
    // iterate over multipart stream
    // Upload "one" file at a time
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filename = sanitize_filename::sanitize(filename);
        let mut filedata = Vec::<Bytes>::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            filedata.push(data);
        }
        let request = StaticfileRequest { filename, filedata };
        response = staticserver.savestaticfile(request).await?;
    }
    return Ok(HttpResponse::Ok().json(response));
}
