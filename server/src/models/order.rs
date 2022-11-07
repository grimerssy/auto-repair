use diesel::Queryable;
use serde::Serialize;

use super::{car::Car, id::Id, service::Service, worker::Worker};

#[derive(Queryable)]
pub struct RawOrder {
    pub id: Id,
    pub service_id: i32,
    pub worker_id: i32,
    pub car_vin: String,
    pub start_time: String,
}

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: Id,
    pub service: Service,
    pub worker: Worker,
    pub car: Car,
    pub start_time: String,
}

pub struct InsertOrder {
    pub specialty_id: Id,
    pub car_vin: String,
    pub start_time: String,
}
