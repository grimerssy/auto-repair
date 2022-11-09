use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::services,
    data::DbPool,
    errors::map::from_diesel_error,
    models::{
        id::{keys::Keys, Id},
        money::Money,
        service::{InsertService, Service},
        time::Time,
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

#[post("/specialties/{worker_id}/{service_id}")]
pub async fn add_for_worker(
    req: HttpRequest,
    path: Path<(Id, Id)>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let (mut worker_id, mut service_id) = path.into_inner();
    worker_id.decode(keys.workers);
    service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    services::add_to_worker(worker_id, service_id, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("/specialties/{worker_id}")]
pub async fn get_for_worker(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<Vec<Service>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut worker_id = path.into_inner();
    worker_id.decode(keys.workers);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = services::get_for_worker(worker_id, conn)
        .await
        .map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("/{worker_id}")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>) -> Result<Json<Vec<Service>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = services::get_all(conn).await.map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("/{id}")]
pub async fn get_by_id(
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<Json<Service>> {
    let mut id = path.into_inner();
    id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut result = services::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;
    result.id.encode(keys.services);
    Ok(Json(result))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[delete("/specialties/{worker_id}/{service_id}")]
pub async fn remove_for_worker(
    req: HttpRequest,
    path: Path<(Id, Id)>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let (mut worker_id, mut service_id) = path.into_inner();
    worker_id.decode(keys.workers);
    service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    services::remove_for_worker(worker_id, service_id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
