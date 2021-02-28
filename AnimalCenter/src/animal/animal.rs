use crate::infrastruct::stringtoObjectId;
use bson::doc;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

use async_graphql::*;

use crate::{
    entity::animalentity::AnimalEntity,
    infrastruct::context::dbcontext::{DBContext, IDbContext},
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AnimalResponse {
    pub id: String,
    pub name: String,
    pub animal_type: Vec<String>,
}

#[Object]
impl AnimalResponse {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn animal_type(&self) -> &Vec<String> {
        &self.animal_type
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn addanimal(&self, animal: Animal) -> Result<AnimalResponse> {
        let mut result = AnimalResponse::default();
        println!("addanimal data is {:?}", animal);
        result.id = "0001".to_owned();
        result.name = animal.name;
        result.animal_type = animal.animal_type;
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryRoot {
    pub name: String,
}
pub type AnimalSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

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
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct Animal {
    pub name: String,
    pub animal_type: Vec<String>,
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(std::time::Duration::from_secs(1)).map(move |_| {
            value += step;
            value
        })
    }
}
