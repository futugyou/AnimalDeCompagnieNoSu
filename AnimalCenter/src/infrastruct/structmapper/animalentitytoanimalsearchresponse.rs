use crate::model::animal::animalmodel::AnimalSearchResponse;
use crate::{entity::animalentity::AnimalEntity, infrastruct::getdefaultdatetime};

impl From<AnimalEntity> for AnimalSearchResponse {
    fn from(animal: AnimalEntity) -> Self {
        let d = animal.birthday;
        let mut birthday = getdefaultdatetime();
        match d {
            Some(a) => birthday = a,
            _ => {}
        }
        AnimalSearchResponse {
            id: animal.id,
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: Some(birthday),
            idcard: animal.idcard,
        }
    }
}
