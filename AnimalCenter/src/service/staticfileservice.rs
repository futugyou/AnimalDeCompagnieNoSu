use crate::infrastruct::config::Config;
use async_std::{
    fs::{self, File},
    path::Path,
    prelude::*,
};
use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;

use crate::{
    infrastruct::custom_error::CustomError,
    model::file::staticfilemodel::{StaticfileRequest, StaticfileResponse},
};

#[async_trait]
pub trait IStaticfileService {
    async fn savestaticfile(
        &self,
        request: StaticfileRequest,
    ) -> Result<StaticfileResponse, CustomError>;
}

pub struct StaticfileService {}

#[async_trait]
impl IStaticfileService for StaticfileService {
    #[tracing::instrument(skip(self))]
    async fn savestaticfile(
        &self,
        request: StaticfileRequest,
    ) -> Result<StaticfileResponse, CustomError> {
        let config = Config::new();
        let address = config.server_address;

        let subfilepath = Utc::now().format("%Y-%m-%d").to_string();
        let access_address = format!("http://{}/{}/{}", address, "images", subfilepath);

        let path_string = format!("{}{}", "fileupload/", subfilepath);
        let path = Path::new(&path_string);
        fs::create_dir_all(path).await?;

        let StaticfileRequest {
            mut filename,
            filedata,
        } = request;
        filename = format!(
            "{}-{:>04}-{}",
            Utc::now().format("%Y%m%d-%H%M%S"),
            rand::thread_rng().gen_range(0001..9999),
            filename
        );

        let access_address = format!("{}/{}", access_address, filename);
        let filepath = format!("{}/{}", path_string, filename);

        let mut f = File::create(filepath.clone()).await?;

        for data in filedata.iter() {
            f.write_all(data).await?;
        }
        f.flush().await?;

        let staticfile = StaticfileResponse {
            name: filename.clone(),
            status: "done".to_owned(),
            thumb_url: format!("{}", access_address),
            url: format!("{}", access_address),
        };
        Ok(staticfile)
    }
}
