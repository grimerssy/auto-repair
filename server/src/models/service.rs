use diesel::Queryable;
use serde::Serialize;

use super::{id::Id, money::Money, time::Time};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: Id,
    pub title: String,
    pub price: Money,
    pub duration: Time,
}
