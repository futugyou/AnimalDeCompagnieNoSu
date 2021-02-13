use crate::model::animal::animalmodel::AnimalClearFakeData;
use bson::{doc, Document};

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
