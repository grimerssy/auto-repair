use diesel::Queryable;
use serde::Serialize;

use super::{contact::Contact, id::Id};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawUser {
    pub contact_id: Id,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: String,
    pub registered_at: String,
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub contact: Contact,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: String,
    pub registered_at: String,
}

pub struct InsertUser {
    pub contact_id: Id,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: String,
}
