use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::DbPool,
    data::{services, sql_to_chrono_format, timestamp},
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
    web::{Data, Json, Path, Query},
    HttpRequest, HttpResponse,
};
use genpdf::{elements, fonts, style};
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddForWorkerRequest {
    worker_id: Id,
    service_id: Id,
}

#[post("/specialties/query")]
pub async fn add_for_worker(
    req: HttpRequest,
    query: Query<AddForWorkerRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut query = query.into_inner();
    query.worker_id.decode(keys.workers);
    query.service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    services::add_to_worker(query.worker_id, query.service_id, conn)
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetForWorkerRequest {
    worker_id: Id,
}

#[get("/specialties/query")]
pub async fn get_for_worker(
    req: HttpRequest,
    query: Query<GetForWorkerRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<Json<Vec<Service>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut query = query.into_inner();
    query.worker_id.decode(keys.workers);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = services::get_for_worker(query.worker_id, conn)
        .await
        .map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
}

#[get("")]
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
pub struct GetByTitleRequest {
    title: String,
}

#[get("/search/title")]
pub async fn get_by_title(
    query: Query<GetByTitleRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Service>>> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut results = services::get_by_title(query.into_inner().title, conn)
        .await
        .map_err(from_diesel_error())?;
    results.iter_mut().for_each(|r| r.id.encode(keys.services));
    Ok(Json(results))
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteForWorkerRequest {
    worker_id: Id,
    service_id: Id,
}

#[delete("/specialties/query")]
pub async fn remove_for_worker(
    req: HttpRequest,
    query: Query<DeleteForWorkerRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut query = query.into_inner();
    query.worker_id.decode(keys.workers);
    query.service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    services::remove_for_worker(query.worker_id, query.service_id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[get("/priceList/pdf")]
pub async fn get_price_list_pdf(db_pool: Data<DbPool>) -> Result<HttpResponse> {
    let conn = &mut retrieve_connection(db_pool).await?;
    let results = services::get_all(conn).await.map_err(from_diesel_error())?;
    let font_family = fonts::from_files("./assets/fonts/Roboto", "Roboto", None).unwrap();
    let mut doc = genpdf::Document::new(font_family);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(15);
    doc.set_page_decorator(decorator);
    doc.set_title("Price list");
    doc.set_line_spacing(1.5);
    doc.set_font_size(14);
    doc.push(elements::StyledElement::new(
        elements::Paragraph::new(format!(
            "Price list for {}",
            chrono::offset::Utc::now()
                .naive_utc()
                .format(&sql_to_chrono_format(timestamp::FORMAT))
        )),
        style::Effect::Bold,
    ));
    doc.push(elements::Break::new(1));
    results.into_iter().for_each(|s| {
        doc.push(elements::StyledElement::new(
            elements::Paragraph::new(s.title.to_uppercase()),
            style::Effect::Italic,
        ));
        doc.push(elements::Paragraph::new(format!("Price: {}", s.price)));
        doc.push(elements::Paragraph::new(format!(
            "Duration: {}",
            s.duration
        )));
        doc.push(elements::Break::new(1));
    });
    let mut buffer = vec![0; 1 << 24];
    doc.render(buffer.as_mut_slice()).unwrap();
    Ok(HttpResponse::Ok()
        .insert_header(("content-type", "application/pdf"))
        .body(buffer))
}
