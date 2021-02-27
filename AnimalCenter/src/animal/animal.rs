use crate::infrastruct::stringtoObjectId;
use bson::doc;
use serde::{Deserialize, Serialize};

use async_graphql::*;

use crate::{
    entity::animalentity::AnimalEntity,
    infrastruct::context::dbcontext::{DBContext, IDbContext},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryRoot {
    pub name: String,
}
pub type AnimalSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
//  {
//     page_num: i32,
//     page_size: i32,
//     query: AnimalQueryParameter,
// }
#[derive(Clone)]
struct AnimalQueryParameter {
    animal_types: Vec<String>,
}

#[Object]
impl QueryRoot {
    async fn name(&self) -> String {
        self.name.to_string()
    }
    async fn name_from_db(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Id of object")] id: String,
    ) -> Result<String> {
        let dbcontext = DBContext {};
        let dbclient = dbcontext.get_db_context().await.unwrap();
        let collection = dbclient
            .database("react-app")
            .collection(AnimalEntity::get_collection_name());
        let oid = stringtoObjectId(&id).unwrap();
        let filter = doc! {"_id":oid};
        let result = collection.find_one(filter, None).await.unwrap().unwrap();
        let mut name = "".to_owned();
        if let Some(n) = result.get("name").and_then(bson::Bson::as_str) {
            name = n.to_string();
        }
        Ok(name)
    }
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub name: String,
    pub animal_type: String,
}
