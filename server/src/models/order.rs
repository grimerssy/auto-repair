use diesel::Queryable;
use serde::Serialize;

use super::{contact::Contact, id::Id, service::Service, timestamp::Timestamp};

#[derive(Queryable)]
pub struct RawOrder {
    pub id: Id,
    pub contact_id: i32,
    pub service_id: i32,
    pub start_time: Timestamp,
    pub car_make: String,
    pub car_model: String,
    pub car_year: i16,
}

#[derive(Queryable, Serialize)]
pub struct Order {
    pub id: Id,
    pub contact: Contact,
    pub service: Service,
    pub start_time: Timestamp,
    pub car_make: String,
    pub car_model: String,
    pub car_year: i16,
}

pub struct InsertOrder {
    pub contact_id: Id,
    pub service_id: Id,
    pub start_time: Timestamp,
    pub car_make: String,
    pub car_model: String,
    pub car_year: i16,
}
