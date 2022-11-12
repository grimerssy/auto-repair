use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::{cars, contacts, orders, DbPool},
    errors::map::from_diesel_error,
    models::{
        car::InsertCar,
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

#[get("/service/time/{service_id}")]
pub async fn get_available_time(
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<Json<Vec<(String, Vec<String>)>>> {
    let mut service_id = path.into_inner();
    service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    orders::get_available_time(service_id, conn)
        .await
        .map(Json)
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    service_id: Id,
    phone_number: String,
    email: Option<String>,
    car_vin: String,
    car_make: String,
    car_model: String,
    car_year: i16,
    start_time: String,
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
                let mut service_id = req_body.service_id;
                service_id.decode(keys.services);
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id = contacts::upsert_returning_id(insert_contact, conn).await?;

                let car_result = cars::get_by_vin(req_body.car_vin.clone(), conn).await;
                if let Ok(car) = car_result {
                    if car.contact.id != contact_id
                        || car.make != req_body.car_make.clone()
                        || car.model != req_body.car_model.clone()
                        || car.year != req_body.car_year
                    {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                } else {
                    let car = InsertCar {
                        contact_id,
                        vin: req_body.car_vin.clone(),
                        make: req_body.car_make.clone(),
                        model: req_body.car_model.clone(),
                        year: req_body.car_year,
                    };
                    cars::insert(car, conn).await?;
                }

                let specialty_id =
                    orders::get_specialty_id(service_id, req_body.start_time.clone(), conn).await?;

                let order = InsertOrder {
                    specialty_id,
                    car_vin: req_body.car_vin.clone(),
                    start_time: req_body.start_time.clone(),
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
        o.service.id.encode(keys.services);
        o.worker.id.encode(keys.workers);
        o.car.contact.id.encode(keys.contacts);
    });

    Ok(Json(orders))
}

#[get("/service/{service_id}")]
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
        o.service.id.encode(keys.services);
        o.worker.id.encode(keys.workers);
        o.car.contact.id.encode(keys.contacts);
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
    order.service.id.encode(keys.services);
    order.worker.id.encode(keys.workers);
    order.car.contact.id.encode(keys.contacts);

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
                let mut service_id = req_body.service_id;
                service_id.decode(keys.services);
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id = contacts::upsert_returning_id(insert_contact, conn).await?;

                let car_result = cars::get_by_vin(req_body.car_vin.clone(), conn).await;
                if let Ok(car) = car_result {
                    if car.contact.id != contact_id
                        || car.make != req_body.car_make.clone()
                        || car.model != req_body.car_model.clone()
                        || car.year != req_body.car_year
                    {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                } else {
                    let car = InsertCar {
                        contact_id,
                        vin: req_body.car_vin.clone(),
                        make: req_body.car_make.clone(),
                        model: req_body.car_model.clone(),
                        year: req_body.car_year,
                    };
                    cars::insert(car, conn).await?;
                }

                let specialty_id =
                    orders::get_specialty_id(service_id, req_body.start_time.clone(), conn).await?;

                let order = InsertOrder {
                    specialty_id,
                    car_vin: req_body.car_vin.clone(),
                    start_time: req_body.start_time.clone(),
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
