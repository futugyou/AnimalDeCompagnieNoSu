use bson::{doc, Document};

use crate::{entity::rescueentity::RescueEntity, model::report::rescue_classification::*};

impl From<RescueClassificationRequest> for Vec<Document> {
    fn from(entity: RescueClassificationRequest) -> Self {
        if entity.rescue_classification == "age" {
            return vec![
                doc! {"$project":doc!{"subyear":doc!{"$subtract":vec![doc!{"$year":"$$NOW"},doc!{"$year": "$birthday"}]}}},
                doc! {"$group":{"_id":"$subyear","count":{"$sum":1}}},
                doc! {"$project":{"_id":0,"classification":{"$toString":"$_id"},"count":1}},
            ];
        } else {
            return vec![
                doc! {"$group":{"_id":"$sub_type","count":{"$sum":1}}},
                doc! {"$project":{"_id":0,"classification":"$_id","count":1}},
            ];
        }
    }
}

impl From<RescueEntity> for RescueClassificationResponse {
    fn from(entity: RescueEntity) -> Self {
        Self {
            classification: entity.classification,
            count: entity.count,
        }
    }
}
