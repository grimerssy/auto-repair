use actix_web::{post, HttpResponse, get};
use actix_web::web::{Json, Data, Path};
use diesel::result::Error;
use serde::Deserialize;

use crate::data::orders::{insert_order, get_all_orders, get_orders_by_service_id};
use crate::errors::map::from_diesel_error;
use crate::models::id::Id;
use crate::models::id::keys::Keys;
use crate::models::order::{InsertOrder, Order};
use crate::models::contact::InsertContact;
use crate::data::contacts::get_contact_id_by_pn_create_if_absent;
use crate::{data::DbPool, errors::ServerError};

use super::retrieve_connection;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MakeOrderRequest {
    phone_number: String,
    email: Option<String>,
    service_id: Id,
    start_time: String,
    car_make: String,
    car_model: String,
    car_year: i16,
}

#[post("")]
pub async fn make_order(
    req_body: Json<MakeOrderRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>)
-> Result<HttpResponse, ServerError>
{
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction().run::<(), Error, _>(|conn| { Box::pin(async move {
        let insert_contact = InsertContact {
            phone_number: req_body.phone_number.clone(),
            email: req_body.email.clone(),
        };
        let contact_id = get_contact_id_by_pn_create_if_absent(insert_contact, conn)
            .await?;

        let mut service_id = req_body.service_id;
        service_id.decode(keys.services);

        let order = InsertOrder {
            contact_id,
            service_id,
            start_time: req_body.start_time.clone(),
            car_make: req_body.car_make.clone(),
            car_model: req_body.car_model.clone(),
            car_year: req_body.car_year,
        };
        insert_order(order, conn).await
    })}).await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("")]
pub async fn get_all(
    db_pool: Data<DbPool>,
    keys: Data<Keys>)
-> Result<Json<Vec<Order>>, ServerError>
{
    let conn = &mut retrieve_connection(db_pool).await?;

    let mut orders = get_all_orders(conn)
        .await
        .map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.contact.id.encode(keys.contacts);
        o.service.id.encode(keys.services);
    });

    Ok(Json(orders))
}

#[get("/{id}")]
pub async fn get_by_service_id(
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>)
-> Result<Json<Vec<Order>>, ServerError>
{
    let mut service_id = path.into_inner();
    service_id.decode(keys.services);

    let conn = &mut retrieve_connection(db_pool).await?;

    let mut orders = get_orders_by_service_id(service_id, conn)
        .await
        .map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.contact.id.encode(keys.contacts);
        o.service.id.encode(keys.services);
    });

    Ok(Json(orders))
}
