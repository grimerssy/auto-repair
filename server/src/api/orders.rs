use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::{contacts, orders, DbPool},
    errors::map::from_diesel_error,
    models::{
        contact::InsertContact,
        id::{keys::Keys, Id},
        order::{InsertOrder, Order},
    },
    JwtCfg,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use diesel::result::Error as DieselError;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    phone_number: String,
    email: Option<String>,
    service_id: Id,
    start_time: String,
    car_make: String,
    car_model: String,
    car_year: i16,
}

#[post("")]
pub async fn create(
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction()
        .run::<(), DieselError, _>(|conn| {
            Box::pin(async move {
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id =
                    contacts::get_id_by_phone_number_create_if_absent(insert_contact, conn).await?;

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
                orders::insert(order, conn).await
            })
        })
        .await
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
    let mut orders = orders::get_all(conn).await.map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.contact.id.encode(keys.contacts);
        o.service.id.encode(keys.services);
    });

    Ok(Json(orders))
}

#[get("/service/{id}")]
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
    let mut orders = orders::get_by_service_id(service_id, conn)
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
pub async fn get_by_id(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Order>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut order = orders::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;
    order.id.encode(keys.orders);
    order.contact.id.encode(keys.contacts);
    order.service.id.encode(keys.services);

    Ok(Json(order))
}

#[put("/update/{id}")]
pub async fn update_by_id(
    req: HttpRequest,
    path: Path<Id>,
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction()
        .run::<(), DieselError, _>(|conn| {
            Box::pin(async move {
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id =
                    contacts::get_id_by_phone_number_create_if_absent(insert_contact, conn).await?;

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
                orders::update_by_id(id, order, conn).await
            })
        })
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[delete("/{id}")]
pub async fn delete_by_id(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;
    orders::delete_by_id(id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
