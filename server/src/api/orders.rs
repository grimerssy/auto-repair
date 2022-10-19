use super::{Result, retrieve_connection, get_claims, check_if_admin};
use crate::{
    data::{
        DbPool,
        orders::{insert_order, get_all_orders, get_orders_by_service_id},
        contacts::get_contact_id_by_pn_create_if_absent,
    },
    errors::map::from_diesel_error,
    models::{
        id::{ Id, keys::Keys},
        order::{InsertOrder, Order},
        contact::InsertContact,
    }, JwtCfg,
};
use actix_web::{
    post,
    HttpResponse,
    get,
    web::{Json, Data, Path}, HttpRequest,
};
use diesel::result::Error as DieselError;
use serde::Deserialize;

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
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction().run::<(), DieselError, _>(|conn| { Box::pin(async move {
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
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Order>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
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
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Order>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
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
