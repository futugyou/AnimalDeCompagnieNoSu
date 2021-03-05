use crate::{
    entity::fileentity::FileEntity, infrastruct::base64convert::vectobase64,
    model::file::filemodel::*,
};

impl From<FileEntity> for FileSearchResponse {
    fn from(file: FileEntity) -> Self {
        FileSearchResponse {
            id: file.id,
            name: file.name,
            ext: file.ext,
            base64src: file.base64src,
        }
    }
}

impl From<FileAddModel> for FileEntity {
    fn from(file: FileAddModel) -> Self {
        let base64 = vectobase64(&file.data, &file.ext);
        FileEntity {
            id: "".to_owned(),
            name: file.name,
            ext: file.ext,
            base64src: base64,
        }
    }
}
