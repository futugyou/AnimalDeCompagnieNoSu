use crate::model::animal::{animalinsertmodel::*, animalsearchmodel::*, animalupdatemodel::*};
use crate::{
    entity::animalentity::AnimalEntity, infrastruct::getdefaultdatetime, model::animal::*,
};

use bson::{doc, Document};

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
            avatar: animal.avatar,
            photoes: animal.photoes,
        }
    }
}

impl From<AnimalSearchRequest> for Document {
    fn from(query: AnimalSearchRequest) -> Self {
        let mut filter = doc! {};
        if query.name != "" {
            filter.insert("name", doc! {"$regex": query.name});
        }
        if query.animal_type.len() > 0 {
            filter.insert(
                "$or",
                vec![
                    doc! {"type":doc!{"$in":&query.animal_type}},
                    doc! {"sub_type":doc!{"$in":&query.animal_type}},
                ],
            );
        }
        filter
    }
}

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
            avatar: animal.avatar,
            photoes: animal.photoes,
        }
    }
}

impl From<AnimalClearFakeData> for Document {
    fn from(_query: AnimalClearFakeData) -> Self {
        doc! {
            "$or":
            vec![
                doc! {"type":doc!{"$in":vec![""]}},
                doc! {"sub_type":doc!{"$in":vec![""]}},
                doc! {"name": "" },
                doc! {"idcard":  "" },
                doc! {"type":  null},
                doc! {"sub_type":null },
                doc! {"name": null },
                doc! {"idcard":  null},
            ],
        }
    }
}

impl From<AnimalEntity> for AnimalUpdateResponse {
    fn from(entity: AnimalEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            idcard: entity.idcard,
            animal_type: entity.animal_type,
            sub_type: entity.sub_type,
            birthday: entity.birthday,
            avatar: entity.avatar,
            photoes: entity.photoes,
        }
    }
}

impl From<&AnimalEntity> for Document {
    fn from(entity: &AnimalEntity) -> Self {
        doc! {
                "name": &entity.name,
                "type": &entity.animal_type,
                "birthday": &entity.birthday.unwrap(),
                "sub_type": &entity.sub_type,
                "idcard": &entity.idcard,
                "avatar": &entity.avatar,
                "photoes": &entity.photoes
        }
    }
}

impl From<AnimalInsertRequest> for AnimalEntity {
    fn from(animal: AnimalInsertRequest) -> Self {
        let d = animal.birthday;
        let mut birthday = getdefaultdatetime();
        match d {
            Some(a) => birthday = a,
            _ => {}
        }
        AnimalEntity {
            id: "".to_owned(),
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: Some(birthday),
            idcard: "".to_owned(),
            avatar: animal.avatar,
            photoes: animal.photoes,
        }
    }
}
impl From<AnimalEntity> for AnimalInsertResponse {
    fn from(entity: AnimalEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            idcard: entity.idcard,
            animal_type: entity.animal_type,
            sub_type: entity.sub_type,
            birthday: entity.birthday,
            avatar: entity.avatar,
            photoes: entity.photoes,
        }
    }
}
