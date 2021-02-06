use crate::model::animal::animalmodel::AnimalSearchRequest;
use bson::{doc, Document};

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
