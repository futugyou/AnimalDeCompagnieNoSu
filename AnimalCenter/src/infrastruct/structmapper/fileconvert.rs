use crate::{entity::fileentity::FileEntity, model::animal::filemodel::FileSearchResponse};

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
