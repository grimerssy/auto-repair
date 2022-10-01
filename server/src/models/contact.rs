use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Contact {
    pub id: i32,
    pub phone_number: String,
    pub email: Option<String>,
}

pub struct InsertContact {
    pub phone_number: String,
    pub email: Option<String>,
}
