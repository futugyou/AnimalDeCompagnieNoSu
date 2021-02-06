use crate::entity::animalentity::AnimalEntity;
use crate::model::animal::animalmodel::AnimalSearchResponse;

impl From<AnimalEntity> for AnimalSearchResponse {
    fn from(animal: AnimalEntity) -> Self {
        AnimalSearchResponse {
            id: animal.id,
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: Some(animal.birthday),
            idcard: animal.idcard,
        }
    }
}
