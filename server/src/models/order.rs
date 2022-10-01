use diesel::Queryable;
use serde::Serialize;

use super::timestamp::Timestamp;

#[derive(Queryable, Serialize)]
pub struct Order {
    pub id: i32,
    pub contact_id: i32,
    pub service_id: i32,
    pub start_time: Timestamp,
    pub car_make: String,
    pub car_model: String,
    pub car_year: i16,
}

pub struct InsertOrder {
    pub contact_id: i32,
    pub service_id: i32,
    pub start_time: Timestamp,
    pub car_make: String,
    pub car_model: String,
    pub car_year: i16,
}
