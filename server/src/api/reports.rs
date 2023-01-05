use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::{reports, DbPool},
    errors::map::from_diesel_error,
    models::{
        id::keys::Keys,
        report_data::{CarsReport, ClientsReport, ServiceReport, WorkHoursReport},
    },
    JwtCfg,
};
use actix_web::{
    get,
    web::{Data, Json},
    HttpRequest,
};

#[get("/services")]
pub async fn get_most_profitable_services_for_month(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<Vec<ServiceReport>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut services = reports::get_most_profitable_services_for_month(conn)
        .await
        .map_err(from_diesel_error())?;
    services.iter_mut().for_each(|s| s.id.encode(keys.services));
    Ok(Json(services))
}

#[get("/clients")]
pub async fn get_most_valuable_clients_for_month(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<Vec<ClientsReport>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut clients = reports::get_most_valuable_clients_for_month(conn)
        .await
        .map_err(from_diesel_error())?;
    clients.iter_mut().for_each(|c| c.id.encode(keys.contacts));
    Ok(Json(clients))
}

#[get("/cars")]
pub async fn get_most_frequent_cars_for_month(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<Vec<CarsReport>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    reports::get_most_frequent_cars_for_month(conn)
        .await
        .map(Json)
        .map_err(from_diesel_error())
}

#[get("/hours")]
pub async fn get_total_work_hours_for_month(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<WorkHoursReport>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    reports::get_total_work_hours_for_month(conn)
        .await
        .map(Json)
        .map_err(from_diesel_error())
}
