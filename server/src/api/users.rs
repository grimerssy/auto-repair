use super::retrieve_connection;
use actix_web::{post, HttpResponse, web::{Data, Json}};
use serde::Deserialize;
use bcrypt::hash;

use crate::{
    data::{
        DbPool,
        contacts::get_contact_id_by_pn_create_if_absent,
        users::insert_user
    },
    models::{user::InsertUser, contact::InsertContact},
    errors::{ServerError, map::to_internal_error},
    BcryptCfg
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    phone_number: String,
    email: Option<String>,
    password: String,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    dob: String,
}


#[post("signup")]
pub async fn signup(
    req_body: Json<SignupRequest>,
    db_pool: Data<DbPool>,
    bcrypt_cfg: Data<BcryptCfg>,
) -> Result<HttpResponse, ServerError> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let insert_contact = InsertContact {
        phone_number: req_body.phone_number.clone(),
        email: req_body.email.clone(),
    };
    let contact_id = get_contact_id_by_pn_create_if_absent(insert_contact, conn)
        .await
        .map_err(to_internal_error())?;
    let password_hash = hash(req_body.password.clone(), bcrypt_cfg.cost)
        .map_err(to_internal_error())?;
    let user = InsertUser {
        contact_id,
        password_hash,
        first_name: req_body.first_name.clone(),
        middle_name: req_body.middle_name.clone(),
        last_name: req_body.last_name.clone(),
        dob: req_body.dob.clone(),
    };
    insert_user(user, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(to_internal_error())
}
