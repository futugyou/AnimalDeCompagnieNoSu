use serde::{Deserialize, Serialize};

use async_graphql::*;
pub struct QueryRoot;
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
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    name: String,
    name_type: String,
}
