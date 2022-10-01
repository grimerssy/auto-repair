use actix_web::{post, HttpResponse, get};
use actix_web::web::{Json, Data, Query};
use diesel::result::Error;
use serde::Deserialize;

use crate::data::orders::{insert_order, get_orders_by_contact_id};
use crate::models::order::{InsertOrder, Order};
use crate::models::contact::InsertContact;
use crate::models::timestamp::Timestamp;
use crate::data::contacts::{get_contact_by_phone_number, insert_contact_returning_id, update_contact_email, get_contact_id_by_phone_number};
use crate::{data::DbPool, errors::{ServerError, map::to_internal_error}};

use super::retrieve_connection;

#[derive(Deserialize)]
pub struct MakeOrderRequest {
    phone_number: String,
    email: Option<String>,
    service_id: i32,
    start_time: Timestamp,
    car_make: String,
    car_model: String,
    car_year: i16,
}

#[post("")]
pub async fn make_order(req_body: Json<MakeOrderRequest>, db_pool: Data<DbPool>)
-> Result<HttpResponse, ServerError>
{
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction().run::<(), Error, _>(|conn| { Box::pin(async move {
        let contact_result = get_contact_by_phone_number(
            req_body.phone_number.clone(), conn).await;

        let contact_id = match contact_result {
            Ok(mut contact) => {
                let id = contact.id;
                if contact.email == None && req_body.email.clone() != None {
                    contact.email = req_body.email.clone();
                    update_contact_email(contact, conn).await?;
                }
                id
            },
            Err(_) => {
                let contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                insert_contact_returning_id(contact, conn).await?
            }
        };

        let order = InsertOrder {
            contact_id,
            service_id: req_body.service_id,
            start_time: req_body.start_time,
            car_make: req_body.car_make.clone(),
            car_model: req_body.car_model.clone(),
            car_year: req_body.car_year,
        };
        insert_order(order, conn).await
    })}).await
        .map(|_| HttpResponse::Created().finish())
        .map_err(to_internal_error())
}

#[derive(Deserialize)]
pub struct GetByContactQuery {
    phone_number: String,
}

#[get("")]
pub async fn get_by_contact(
    query: Query<GetByContactQuery>, db_pool: Data<DbPool>)
-> Result<Json<Vec<Order>>, ServerError>
{
    let conn = &mut retrieve_connection(db_pool).await?;

    let contact_id =
        get_contact_id_by_phone_number(query.phone_number.clone(), conn)
        .await
        .map_err(to_internal_error())?;

    let orders = get_orders_by_contact_id(contact_id, conn)
        .await
        .map_err(to_internal_error())?;

    Ok(Json(orders))
}