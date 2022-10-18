use super::{Result, retrieve_connection};
use crate::{
    data::DbPool,
    models::{service::Service, id::{keys::Keys, Id}},
    data::services::{get_all_services, get_service_by_id},
    errors::map::from_diesel_error,
};
use actix_web::{web::{Data, Json, Path}, get};

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>, keys: Data<Keys>,
) -> Result<Json<Vec<Service>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = get_all_services(conn).await.map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("/{id}")]
pub async fn get_by_id(path: Path<Id>, db_pool: Data<DbPool>, keys: Data<Keys>,
) -> Result<Json<Service>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut id = path.into_inner();
    id.decode(keys.services);
    let mut result = get_service_by_id(id, conn).await.map_err(from_diesel_error())?;
    result.id.encode(keys.services);
    Ok(Json(result))
}
