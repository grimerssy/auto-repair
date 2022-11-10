use super::{retrieve_connection, Claims, Result};
use crate::{
    data::{
        contacts::*,
        users::{get_by_contact_id, insert},
        DbPool,
    },
    errors::{
        map::{from_diesel_error, to_internal_error},
        Error,
    },
    models::{contact::InsertContact, id::keys::Keys, user::InsertUser},
    BcryptCfg, JwtCfg,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use bcrypt::{hash, verify};
use jsonwebtoken::{self, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    phone_number: String,
    email: Option<String>,
    password: String,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

#[post("/signup")]
pub async fn signup(
    req_body: Json<SignupRequest>,
    db_pool: Data<DbPool>,
    bcrypt_cfg: Data<BcryptCfg>,
) -> Result<HttpResponse> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let insert_contact = InsertContact {
        phone_number: req_body.phone_number.clone(),
        email: req_body.email.clone(),
    };
    let contact_id = upsert_returning_id(insert_contact, conn)
        .await
        .map_err(from_diesel_error())?;
    let password_hash =
        hash(req_body.password.clone(), bcrypt_cfg.cost).map_err(to_internal_error())?;
    let user = InsertUser {
        contact_id,
        password_hash,
        role: "user".into(),
        first_name: req_body.first_name.clone(),
        middle_name: req_body.middle_name.clone(),
        last_name: req_body.last_name.clone(),
        date_of_birth: req_body.date_of_birth.clone(),
    };
    insert(user, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    phone_number: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    access: String,
}

#[post("/login")]
pub async fn login(
    req_body: Json<LoginRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Tokens>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let contact_id = if let Some(phone_number) = req_body.phone_number.clone() {
        get_id_by_phone_number(phone_number, conn)
            .await
            .map_err(from_diesel_error())
    } else if let Some(email) = req_body.email.clone() {
        get_id_by_email(email, conn)
            .await
            .map_err(from_diesel_error())
    } else {
        Err(Error::BadRequest(
            "must provide either phone number or email".into(),
        ))
    }?;
    let user = get_by_contact_id(contact_id, conn)
        .await
        .map_err(from_diesel_error())?;
    let is_password_valid = verify(req_body.password.clone(), user.password_hash.as_ref())
        .map_err(to_internal_error())?;
    if !is_password_valid {
        return Err(Error::InvalidPassword);
    }
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(jwt_cfg.access_sec_ttl))
        .unwrap_or_default()
        .timestamp_nanos();
    let mut contact_id = user.contact_id;
    contact_id.encode(keys.contacts);
    let claims = Claims {
        sub: contact_id,
        role: user.role,
        exp,
    };
    let access = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_cfg.secret.as_ref()),
    )
    .map_err(to_internal_error())?;

    Ok(Json(Tokens { access }))
}
