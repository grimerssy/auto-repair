use diesel::Queryable;
use serde::Serialize;

use super::id::Id;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: Id,
    pub phone_number: String,
    pub email: Option<String>,
}

pub struct InsertContact {
    pub phone_number: String,
    pub email: Option<String>,
}
