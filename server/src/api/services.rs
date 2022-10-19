use super::{Result, retrieve_connection, check_if_admin, get_claims};
use crate::{
    data::DbPool,
    models::{service::{Service, InsertService}, id::{keys::Keys, Id}, money::Money, time::Time},
    data::services,
    errors::map::from_diesel_error, JwtCfg,
};
use actix_web::{web::{Data, Json, Path}, get, put, HttpRequest, HttpResponse, delete, post};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateRequest {
    title: String,
    price: Money,
    duration: Time,
}

#[post("")]
pub async fn create(
    req: HttpRequest,
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let service = InsertService {
        title: req_body.title.clone(),
        price: req_body.price,
        duration: req_body.duration,
    };
    services::insert(service, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>,
) -> Result<Json<Vec<Service>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = services::get_all(conn).await.map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("/{id}")]
pub async fn get_by_id(path: Path<Id>, db_pool: Data<DbPool>, keys: Data<Keys>,
) -> Result<Json<Service>> {
    let mut id = path.into_inner();
    id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut result = services::get_by_id(id, conn).await.map_err(from_diesel_error())?;
    result.id.encode(keys.services);
    Ok(Json(result))
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UpdateRequest {
    title: String,
    price: Money,
    duration: Time,
}

#[put("/{id}")]
pub async fn update_by_id(
    req: HttpRequest,
    path: Path<Id>,
    req_body: Json<UpdateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    let service = Service {
        id,
        title: req_body.title.clone(),
        price: req_body.price,
        duration: req_body.duration,
    };
    services::update_by_id(service, conn)
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
    id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    services::delete_by_id(id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
