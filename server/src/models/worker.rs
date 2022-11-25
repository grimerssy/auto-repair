use diesel::Queryable;
use serde::Serialize;

use super::id::Id;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Worker {
    pub id: Id,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: String,
    pub start_time: String,
    pub end_time: String,
}

pub struct InsertWorker {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: String,
    pub start_time: String,
    pub end_time: String,
}
