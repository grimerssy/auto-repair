use diesel::Queryable;
use serde::Serialize;

use super::{contact::Contact, id::Id};

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCar {
    pub vin: String,
    pub contact_id: Id,
    pub make: String,
    pub model: String,
    pub year: i16,
}

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub vin: String,
    pub contact: Contact,
    pub make: String,
    pub model: String,
    pub year: i16,
}

pub struct InsertCar {
    pub vin: String,
    pub contact_id: Id,
    pub make: String,
    pub model: String,
    pub year: i16,
}
