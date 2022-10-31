use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::{contacts, DbPool},
    errors::map::from_diesel_error,
    models::{
        contact::{Contact, InsertContact},
        id::{keys::Keys, Id},
    },
    JwtCfg,
};
use actix_web::{
    delete, get, put,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;

#[get("")]
pub async fn get_all(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Contact>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut contacts = contacts::get_all(conn).await.map_err(from_diesel_error())?;

    contacts.iter_mut().for_each(|c| c.id.encode(keys.contacts));
    Ok(Json(contacts))
}

#[get("/self")]
pub async fn get_self(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Contact>> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut id = claims.sub;
    id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut contact = contacts::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;

    contact.id.encode(keys.contacts);
    Ok(Json(contact))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    phone_number: String,
    email: Option<String>,
}

#[put("/self")]
pub async fn update_self(
    req: HttpRequest,
    req_body: Json<UpdateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut id = claims.sub;
    id.decode(keys.contacts);
    let contact = InsertContact {
        phone_number: req_body.phone_number.clone(),
        email: req_body.email.clone(),
    };
    let conn = &mut retrieve_connection(db_pool).await?;
    contacts::update_by_id(id, contact, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[get("/{id}")]
pub async fn get_by_id(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Contact>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut contact = contacts::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;

    contact.id.encode(keys.contacts);
    Ok(Json(contact))
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
    id.decode(keys.contacts);
    let contact = InsertContact {
        phone_number: req_body.phone_number.clone(),
        email: req_body.email.clone(),
    };
    let conn = &mut retrieve_connection(db_pool).await?;
    contacts::update_by_id(id, contact, conn)
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
    id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    contacts::delete_by_id(id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}
