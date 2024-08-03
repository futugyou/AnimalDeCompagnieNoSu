use bson::{doc, Document};
use chrono::DateTime;
use chrono::Utc;

use crate::animal::{
    animalinsertmodel::*, animalsearchmodel::*, animalupdatemodel::*, AnimalClearFakeData,
};
use entity::animalentity::AnimalEntity;
use tool::*;

impl From<AnimalUpdateRequest> for AnimalEntity {
    fn from(animal: AnimalUpdateRequest) -> Self {
        let mut birthday = getdefaultdatetime();
        if let Some(data) = animal.birthday {
            birthday = data;
        }
        let mut rescue_date: DateTime<Utc> = getutcnowwithformat();
        if let Some(data) = animal.rescue_date {
            rescue_date = data;
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
            rescue_date: Some(rescue_date),
        }
    }
}

impl From<AnimalSearchRequest> for Document {
    fn from(query: AnimalSearchRequest) -> Self {
        let mut filter = doc! {};
        if query.name != "" {
            filter.insert("name", doc! {"$regex": query.name});
        }
        if query.animal_type.len() > 0 && query.animal_type[0] != "" {
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
        // let mut birthday: Option<DateTime<Utc>> = None;
        // if let Some(data) = animal.birthday {
        //     birthday = Some(data);
        // }
        // let mut rescue_date: Option<DateTime<Utc>> = None;
        // if let Some(data) = animal.rescue_date {
        //     rescue_date = Some(data);
        // }
        AnimalSearchResponse {
            id: animal.id,
            name: animal.name,
            animal_type: animal.animal_type,
            sub_type: animal.sub_type,
            birthday: animal.birthday,
            idcard: animal.idcard,
            avatar: animal.avatar,
            photoes: animal.photoes,
            rescue_date: animal.rescue_date,
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
            rescue_date: entity.rescue_date,
        }
    }
}

impl From<AnimalInsertRequest> for AnimalEntity {
    fn from(animal: AnimalInsertRequest) -> Self {
        let mut birthday = getdefaultdatetime();
        if let Some(data) = animal.birthday {
            birthday = data;
        }
        let mut rescue_date = getutcnowwithformat();
        if let Some(data) = animal.rescue_date {
            rescue_date = data;
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
            rescue_date: Some(rescue_date),
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
            rescue_date: entity.rescue_date,
        }
    }
}
