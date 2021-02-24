use actix_multipart::Multipart;
use actix_web::{web, HttpRequest};
use actix_web::{Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

use crate::model::animal::filemodel::*;
use crate::service::fileuploadservice::*;

pub async fn post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let service = FileService::new().await;
    let mut models = Vec::new();
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
        models.push(FileAddModel {
            name: filepath,
            ext: ext,
            data: file_data,
        })
    }
    let resul = service.addfiles(models).await?;
    Ok(HttpResponse::Ok().json(resul))
}
pub async fn get(item: Option<web::Json<Vec<String>>>, _req: HttpRequest) -> HttpResponse {
    let service = FileService::new().await;
    let mut ids = Vec::new();
    match item {
        Some(i) => {
            ids = i.into_inner();
        }
        None => {}
    };
    let result = service.find_file_by_ids(ids).await;
    HttpResponse::Ok().json(result)
}
