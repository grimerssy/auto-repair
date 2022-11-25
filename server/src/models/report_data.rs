use diesel::{sql_types, QueryableByName};
use serde::Serialize;

use super::{id::Id, money::Money, time::Time};

#[derive(Serialize, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReport {
    #[diesel(sql_type = sql_types::Integer)]
    pub id: Id,
    #[diesel(sql_type = sql_types::Text)]
    pub title: String,
    #[diesel(sql_type = sql_types::Money)]
    pub price: Money,
    #[diesel(sql_type = sql_types::Time)]
    pub duration: Time,
    #[diesel(sql_type = sql_types::BigInt)]
    pub order_count: i64,
}
#[derive(Serialize, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientsReport {
    #[diesel(sql_type = sql_types::Integer)]
    pub id: Id,
    #[diesel(sql_type = sql_types::Text)]
    pub phone_number: String,
    #[diesel(sql_type = sql_types::Nullable::<sql_types::Text>)]
    pub email: Option<String>,
    #[diesel(sql_type = sql_types::Nullable::<sql_types::Text>)]
    pub first_name: Option<String>,
    #[diesel(sql_type = sql_types::Nullable::<sql_types::Text>)]
    pub middle_name: Option<String>,
    #[diesel(sql_type = sql_types::Nullable::<sql_types::Text>)]
    pub last_name: Option<String>,
    #[diesel(sql_type = sql_types::Nullable::<sql_types::Text>)]
    pub date_of_birth: Option<String>,
    #[diesel(sql_type = sql_types::BigInt)]
    pub order_count: i64,
}

#[derive(Serialize, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CarsReport {
    #[diesel(sql_type = sql_types::Text)]
    pub make: String,
    #[diesel(sql_type = sql_types::Text)]
    pub model: String,
    #[diesel(sql_type = sql_types::Int2)]
    pub year: i16,
    #[diesel(sql_type = sql_types::BigInt)]
    pub order_count: i64,
}

#[derive(Serialize, QueryableByName, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkHoursReport {
    #[diesel(sql_type = sql_types::Integer)]
    pub hours: i32,
}
