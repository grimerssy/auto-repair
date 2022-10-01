use actix_web::{web::{Data, Json}, get};

use super::to_server_error;
use crate::{data::DbPool, models::service::Service, data::services::get_all_services};
use crate::errors::ServerError;

use super::retrieve_connection;

#[get("")]
pub async fn get_all(db_pool: Data<DbPool>) -> Result<Json<Vec<Service>>, ServerError> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let results = get_all_services(conn).await.map_err(to_server_error())?;
    Ok(Json(results))
}
