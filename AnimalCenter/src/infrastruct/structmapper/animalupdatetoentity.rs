use crate::{entity::animalentity::AnimalEntity, model::animal::animalmodel::AnimalUpdateRequest};

impl From<AnimalUpdateRequest> for AnimalEntity {
    fn from(animal: AnimalUpdateRequest) -> Self {
        AnimalEntity {
            id: animal.id,
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: animal.birthday,
            idcard: String::from(""),
        }
    }
}
