use actix_web::{web::{Data, Json, Path}, get};

use super::to_server_error;
use crate::{data::DbPool, models::{service::Service, id::{keys::Keys, Id}}, data::services::{get_all_services, get_service_by_id}};
use crate::errors::ServerError;

use super::retrieve_connection;

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>)
-> Result<Json<Vec<Service>>, ServerError> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = get_all_services(conn).await.map_err(to_server_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("/{id}")]
pub async fn get_by_id(path: Path<Id>, db_pool: Data<DbPool>, keys: Data<Keys>)
-> Result<Json<Service>, ServerError> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut id = path.into_inner();
    id.decode(keys.services);
    let mut result = get_service_by_id(id, conn).await.map_err(to_server_error())?;
    result.id.encode(keys.services);
    Ok(Json(result))
}
