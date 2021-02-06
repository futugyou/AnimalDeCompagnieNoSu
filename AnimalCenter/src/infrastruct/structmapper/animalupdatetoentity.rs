use crate::infrastruct::getdefaultdatetime;
use crate::{entity::animalentity::AnimalEntity, model::animal::animalmodel::AnimalUpdateRequest};

impl From<AnimalUpdateRequest> for AnimalEntity {
    fn from(animal: AnimalUpdateRequest) -> Self {
        let d = animal.birthday;
        let mut birthday = getdefaultdatetime();
        match d {
            Some(a) => birthday = a,
            _ => {}
        }
        AnimalEntity {
            id: animal.id,
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: Some(birthday),
            idcard: String::from(""),
        }
    }
}
