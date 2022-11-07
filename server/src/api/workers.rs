use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::workers,
    data::DbPool,
    errors::map::from_diesel_error,
    models::{
        id::{keys::Keys, Id},
        worker::{InsertWorker, Worker},
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
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
    start_time: String,
    end_time: String,
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
    let worker = InsertWorker {
        first_name: req_body.first_name.clone(),
        middle_name: req_body.middle_name.clone(),
        last_name: req_body.last_name.clone(),
        date_of_birth: req_body.date_of_birth.clone(),
        start_time: req_body.start_time.clone(),
        end_time: req_body.end_time.clone(),
    };
    workers::insert(worker, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>) -> Result<Json<Vec<Worker>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = workers::get_all(conn).await.map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.workers));
    Ok(Json(results))
}

#[get("/{id}")]
pub async fn get_by_id(
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<Json<Worker>> {
    let mut id = path.into_inner();
    id.decode(keys.workers);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut result = workers::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;
    result.id.encode(keys.workers);
    Ok(Json(result))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
    start_time: String,
    end_time: String,
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
    id.decode(keys.workers);
    let conn = &mut retrieve_connection(db_pool).await?;
    let worker = Worker {
        id,
        first_name: req_body.first_name.clone(),
        middle_name: req_body.middle_name.clone(),
        last_name: req_body.last_name.clone(),
        date_of_birth: req_body.date_of_birth.clone(),
        start_time: req_body.start_time.clone(),
        end_time: req_body.end_time.clone(),
    };
    workers::update_by_id(worker, conn)
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
    id.decode(keys.workers);
    let conn = &mut retrieve_connection(db_pool).await?;
    workers::delete_by_id(id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
