use crate::infrastruct::config::Config;
use crate::model::file::staticfilemodel::StaticfileResponse;
use async_std::fs::{self, File};
use async_std::prelude::*;
use chrono::Utc;
use rand::Rng;
use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

pub async fn post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut response = Vec::<StaticfileResponse>::new();

    let config = Config::new();
    let address = config.server_address;

    let subfilepath = Utc::now().format("%Y-%m-%d").to_string();
    let access_address = format!("http://{}/{}/{}", address, "images", subfilepath);

    let path_string = format!("{}{}", "fileupload/", subfilepath);
    let path = Path::new(&path_string);
    fs::create_dir_all(path).await?;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let mut filename = sanitize_filename::sanitize(filename);
        filename = format!(
            "{}-{:>04}-{}",
            Utc::now().format("%Y%m%d-%H%M%S"),
            rand::thread_rng().gen_range(0001..9999),
            filename
        );
        let access_address = format!("{}/{}", access_address, filename);
        let filepath = format!("{}/{}", path_string, filename);

        let mut f = File::create(filepath.clone()).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
        f.flush().await?;
        let staticfile = StaticfileResponse {
            name: filename.clone(),
            status: "done".to_owned(),
            thumb_url: format!("{}", access_address),
            url: format!("{}", access_address),
        };
        response.push(staticfile);
    }
    Ok(HttpResponse::Ok().json(response))
}
