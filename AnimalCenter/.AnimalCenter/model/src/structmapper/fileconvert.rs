use crate::file::filemodel::{FileAddModel, FileSearchResponse};
use entity::fileentity::FileEntity;
use tool::base64convert::vectobase64;

impl From<FileEntity> for FileSearchResponse {
    fn from(file: FileEntity) -> Self {
        FileSearchResponse {
            id: file.id,
            name: file.name,
            ext: file.ext,
            base64src: file.base64src,
            uploaddate: file.uploaddate,
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
            uploaddate: file.uploaddate,
        }
    }
}
