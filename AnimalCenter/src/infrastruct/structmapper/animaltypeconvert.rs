use crate::{
    entity::animaltypeentity::AnimalTypeEntity,
    model::animal::animaltypemodel::{
        AnimalTypeSearchRequest, AnimalTypeSearchResponse, AnimalTypeUpdateRequest,
    },
};
use bson::{doc, Document};

impl From<AnimalTypeSearchRequest> for Document {
    fn from(query: AnimalTypeSearchRequest) -> Self {
        let mut filter = doc! {};
        if query.animal_type.len() > 0 {
            filter.insert("type", doc! {"$in":&query.animal_type});
        }
        filter
    }
}
impl From<AnimalTypeEntity> for AnimalTypeSearchResponse {
    fn from(entity: AnimalTypeEntity) -> Self {
        Self {
            id: entity.id,
            pid: entity.pid,
            animal_type: entity.animal_type,
        }
    }
}
impl From<AnimalTypeUpdateRequest> for AnimalTypeEntity {
    fn from(request: AnimalTypeUpdateRequest) -> Self {
        Self {
            id: request.id,
            pid: request.pid,
            animal_type: request.animal_type,
        }
    }
}
