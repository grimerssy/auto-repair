use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::cars,
    data::DbPool,
    errors::map::from_diesel_error,
    models::{
        car::{Car, InsertCar},
        id::{keys::Keys, Id},
    },
    JwtCfg,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    vin: String,
    contact_id: Id,
    make: String,
    model: String,
    year: i16,
}

#[post("")]
pub async fn create(
    req: HttpRequest,
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut contact_id = req_body.contact_id;
    contact_id.decode(keys.contacts);
    let car = InsertCar {
        vin: req_body.vin.clone(),
        contact_id,
        make: req_body.make.clone(),
        model: req_body.model.clone(),
        year: req_body.year,
    };
    cars::insert(car, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateForSelfRequest {
    vin: String,
    make: String,
    model: String,
    year: i16,
}

#[post("/self")]
pub async fn create_for_self(
    req: HttpRequest,
    req_body: Json<CreateForSelfRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    let car = InsertCar {
        vin: req_body.vin.clone(),
        contact_id,
        make: req_body.make.clone(),
        model: req_body.model.clone(),
        year: req_body.year,
    };
    cars::insert(car, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>) -> Result<Json<Vec<Car>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = cars::get_all(conn).await.map_err(from_diesel_error())?;
    results
        .iter_mut()
        .for_each(|r| r.contact.id.encode(keys.contacts));
    Ok(Json(results))
}

#[get("/self")]
pub async fn get_for_self(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Car>>> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = cars::get_by_contact_id(contact_id, conn)
        .await
        .map_err(from_diesel_error())?;
    results
        .iter_mut()
        .for_each(|r| r.contact.id.encode(keys.contacts));
    Ok(Json(results))
}

#[get("/self/{vin}")]
pub async fn get_by_vin_for_self(
    req: HttpRequest,
    path: Path<String>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Car>> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut result = cars::get_by_vin_for_self(vin, contact_id, conn)
        .await
        .map_err(from_diesel_error())?;
    result.contact.id.encode(keys.contacts);
    Ok(Json(result))
}

#[get("/{vin}")]
pub async fn get_by_vin(
    req: HttpRequest,
    path: Path<String>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Car>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut result = cars::get_by_vin(vin, conn)
        .await
        .map_err(from_diesel_error())?;
    result.contact.id.encode(keys.contacts);
    Ok(Json(result))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    vin: String,
    make: String,
    model: String,
    year: i16,
}

#[put("/self/{vin}")]
pub async fn update_by_vin_for_self(
    req: HttpRequest,
    path: Path<String>,
    req_body: Json<UpdateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    let service = InsertCar {
        vin: req_body.vin.clone(),
        make: req_body.make.clone(),
        model: req_body.model.clone(),
        year: req_body.year,
        contact_id,
    };
    cars::update_by_vin_for_self(vin, service, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[put("/{vin}")]
pub async fn update_by_vin(
    req: HttpRequest,
    path: Path<String>,
    req_body: Json<UpdateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    let service = InsertCar {
        vin: req_body.vin.clone(),
        make: req_body.make.clone(),
        model: req_body.model.clone(),
        year: req_body.year,
        contact_id: Id::Raw(0),
    };
    cars::update_by_vin(vin, service, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[delete("/self/{vin}")]
pub async fn delete_by_vin_for_self(
    req: HttpRequest,
    path: Path<String>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    cars::delete_by_vin_for_self(vin, contact_id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[delete("/{vin}")]
pub async fn delete_by_vin(
    req: HttpRequest,
    path: Path<String>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let vin = path.into_inner();
    let conn = &mut retrieve_connection(db_pool).await?;
    cars::delete_by_vin(vin, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
