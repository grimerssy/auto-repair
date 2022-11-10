use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::DbPool,
    errors::{map::from_diesel_error, Error},
    JwtCfg,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use diesel::{prelude::*, sql_query};
use diesel_async::RunQueryDsl;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlRequest {
    query: String,
}

#[derive(QueryableByName)]
pub struct JsonData {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub data: String,
}

#[post("")]
pub async fn do_sql(
    req: HttpRequest,
    req_body: Json<SqlRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut query = req_body.query.clone();
    let query_split = query.split(' ').collect::<Vec<&str>>();
    let conn = &mut retrieve_connection(db_pool).await?;
    if query_split[0] == "select" {
        let mut chars = query.chars();
        if chars.next_back().unwrap() == ';' {
            query = chars.as_str().to_owned();
        }
        sql_query(format!(
            "select json_agg(\"table\") as data from ({}) as \"table\";",
            query
        ))
        .load::<JsonData>(conn)
        .await
        .map(|x| HttpResponse::Ok().body(x[0].data.clone()))
        .map_err(from_diesel_error())
        .map_err(|e| match e {
            Error::Internal(msg) => Error::BadRequest(msg),
            _ => e,
        })
    } else {
        sql_query(query)
            .execute(conn)
            .await
            .map(|_| HttpResponse::Ok().finish())
            .map_err(from_diesel_error())
    }
}
