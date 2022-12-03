use super::{check_if_admin, get_claims, retrieve_connection, Result};
use crate::{
    data::{cars, contacts, orders, sql_to_chrono_format, timestamp::FORMAT, DbPool},
    errors::{map::from_diesel_error, Error},
    models::{
        car::InsertCar,
        contact::InsertContact,
        id::{keys::Keys, Id},
        order::{InsertOrder, Order},
    },
    JwtCfg,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    HttpRequest, HttpResponse,
};
use diesel::result::Error as DieselError;
use serde::Deserialize;

#[get("/service/time/{service_id}")]
pub async fn get_available_time(
    path: Path<Id>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<Json<Vec<(String, Vec<String>)>>> {
    let mut service_id = path.into_inner();
    service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    orders::get_available_time(service_id, conn)
        .await
        .map(Json)
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    service_id: Id,
    phone_number: String,
    email: Option<String>,
    car_vin: String,
    car_make: String,
    car_model: String,
    car_year: i16,
    start_time: String,
}

#[post("")]
pub async fn create(
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction()
        .run::<(), DieselError, _>(|conn| {
            Box::pin(async move {
                let mut service_id = req_body.service_id;
                service_id.decode(keys.services);
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id = contacts::upsert_returning_id(insert_contact, conn).await?;

                let car_result = cars::get_by_vin(req_body.car_vin.clone(), conn).await;
                if let Ok(car) = car_result {
                    if car.contact.id != contact_id
                        || car.make != req_body.car_make.clone()
                        || car.model != req_body.car_model.clone()
                        || car.year != req_body.car_year
                    {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                } else {
                    let car = InsertCar {
                        contact_id,
                        vin: req_body.car_vin.clone(),
                        make: req_body.car_make.clone(),
                        model: req_body.car_model.clone(),
                        year: req_body.car_year,
                    };
                    cars::insert(car, conn).await?;
                }

                let specialty_id =
                    orders::get_specialty_id(service_id, req_body.start_time.clone(), conn).await?;

                let order = InsertOrder {
                    specialty_id,
                    car_vin: req_body.car_vin.clone(),
                    start_time: req_body.start_time.clone(),
                };
                orders::insert(order, conn).await
            })
        })
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(from_diesel_error())
}

#[get("")]
pub async fn get_all(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Order>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut orders = orders::get_all(conn).await.map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.service.id.encode(keys.services);
        o.worker.id.encode(keys.workers);
        o.car.contact.id.encode(keys.contacts);
    });

    Ok(Json(orders))
}

#[get("/self/all")]
pub async fn get_all_for_self(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Order>>> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut contact_id = claims.sub;
    contact_id.decode(keys.contacts);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut orders = orders::get_all_for_self(contact_id, conn)
        .await
        .map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.service.id.encode(keys.services);
        o.worker.id.encode(keys.workers);
        o.car.contact.id.encode(keys.contacts);
    });

    Ok(Json(orders))
}

#[get("/service/{service_id}")]
pub async fn get_by_service_id(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Vec<Order>>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut service_id = path.into_inner();
    service_id.decode(keys.services);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut orders = orders::get_by_service_id(service_id, conn)
        .await
        .map_err(from_diesel_error())?;
    orders.iter_mut().for_each(|o| {
        o.id.encode(keys.orders);
        o.service.id.encode(keys.services);
        o.worker.id.encode(keys.workers);
        o.car.contact.id.encode(keys.contacts);
    });

    Ok(Json(orders))
}

#[get("/{id}")]
pub async fn get_by_id(
    req: HttpRequest,
    path: Path<Id>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<Json<Order>> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;
    let mut order = orders::get_by_id(id, conn)
        .await
        .map_err(from_diesel_error())?;
    order.id.encode(keys.orders);
    order.service.id.encode(keys.services);
    order.worker.id.encode(keys.workers);
    order.car.contact.id.encode(keys.contacts);

    Ok(Json(order))
}

#[put("/update/{id}")]
pub async fn update_by_id(
    req: HttpRequest,
    path: Path<Id>,
    req_body: Json<CreateRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut id = path.into_inner();
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;

    conn.build_transaction()
        .run::<(), DieselError, _>(|conn| {
            Box::pin(async move {
                let mut service_id = req_body.service_id;
                service_id.decode(keys.services);
                let insert_contact = InsertContact {
                    phone_number: req_body.phone_number.clone(),
                    email: req_body.email.clone(),
                };
                let contact_id = contacts::upsert_returning_id(insert_contact, conn).await?;

                let car_result = cars::get_by_vin(req_body.car_vin.clone(), conn).await;
                if let Ok(car) = car_result {
                    if car.contact.id != contact_id
                        || car.make != req_body.car_make.clone()
                        || car.model != req_body.car_model.clone()
                        || car.year != req_body.car_year
                    {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                } else {
                    let car = InsertCar {
                        contact_id,
                        vin: req_body.car_vin.clone(),
                        make: req_body.car_make.clone(),
                        model: req_body.car_model.clone(),
                        year: req_body.car_year,
                    };
                    cars::insert(car, conn).await?;
                }

                let specialty_id =
                    orders::get_specialty_id(service_id, req_body.start_time.clone(), conn).await?;

                let order = InsertOrder {
                    specialty_id,
                    car_vin: req_body.car_vin.clone(),
                    start_time: req_body.start_time.clone(),
                };
                orders::update_by_id(id, order, conn).await
            })
        })
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
    id.decode(keys.orders);
    let conn = &mut retrieve_connection(db_pool).await?;
    orders::delete_by_id(id, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdArrayRequest {
    ids: String,
}

#[delete("")]
pub async fn delete_by_ids(
    req: HttpRequest,
    query: Query<IdArrayRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut ids = query
        .into_inner()
        .ids
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect::<Vec<Id>>();
    ids.iter_mut().for_each(|id| id.decode(keys.orders));
    let conn = &mut retrieve_connection(db_pool).await?;
    orders::delete_by_ids(ids, conn)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(from_diesel_error())
}

#[get("/self/receipt")]
pub async fn get_receipt_for_self(
    req: HttpRequest,
    query: Query<IdArrayRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    let claims = get_claims(&req, &jwt_cfg.secret).await?;
    let mut id = claims.sub;
    id.decode(keys.contacts);
    let mut ids = query
        .into_inner()
        .ids
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect::<Vec<Id>>();
    ids.iter_mut().for_each(|id| id.decode(keys.orders));
    let conn = &mut retrieve_connection(db_pool).await?;
    let results = orders::get_vec_by_ids(ids, conn)
        .await
        .map_err(from_diesel_error())?;
    if results.is_empty() {
        return Err(Error::NotFound);
    }
    for order in &results {
        if order.car.contact.id != id {
            return Err(Error::BadRequest(
                "Requested order does not belong to sender".into(),
            ));
        }
    }
    Ok(HttpResponse::Ok()
        .insert_header(("content-type", "application/pdf"))
        .body(get_receipt_pdf(results)))
}

#[get("/admin/receipt")]
pub async fn get_receipt(
    req: HttpRequest,
    query: Query<IdArrayRequest>,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
    keys: Data<Keys>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let mut ids = query
        .into_inner()
        .ids
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect::<Vec<Id>>();
    ids.iter_mut().for_each(|id| id.decode(keys.orders));
    let conn = &mut retrieve_connection(db_pool).await?;
    let results = orders::get_vec_by_ids(ids, conn)
        .await
        .map_err(from_diesel_error())?;
    if results.is_empty() {
        return Err(Error::NotFound);
    }
    Ok(HttpResponse::Ok()
        .insert_header(("content-type", "application/pdf"))
        .body(get_receipt_pdf(results)))
}

fn get_receipt_pdf(data: Vec<Order>) -> Vec<u8> {
    use genpdf::{elements, fonts, style};
    let font_family = fonts::from_files("./assets/fonts/Roboto", "Roboto", None).unwrap();
    let mut doc = genpdf::Document::new(font_family);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(15);
    doc.set_page_decorator(decorator);
    doc.set_title("Receipt");
    doc.set_line_spacing(1.5);
    doc.set_font_size(14);
    doc.push(elements::StyledElement::new(
        elements::Paragraph::new(format!(
            "Receipt for {}",
            chrono::offset::Utc::now()
                .naive_utc()
                .format(&sql_to_chrono_format(FORMAT))
        )),
        style::Effect::Bold,
    ));
    doc.push(elements::Break::new(1));
    doc.push(elements::Paragraph::new("Orders: "));
    data.clone().into_iter().for_each(|o| {
        doc.push(elements::Break::new(1));
        doc.push(elements::StyledElement::new(
            elements::Paragraph::new(format!("  {}", o.service.title.to_uppercase())),
            style::Effect::Italic,
        ));
        doc.push(elements::Paragraph::new(format!(
            "  price: {}",
            o.service.price
        )));
        doc.push(elements::Paragraph::new(format!(
            "  duration: {}",
            o.service.duration
        )));
        doc.push(elements::Paragraph::new(format!(
            "  start time: {}",
            o.start_time
        )));
        doc.push(elements::Paragraph::new("  car:"));
        doc.push(elements::Paragraph::new(format!("    vin: {}", o.car.vin)));
        doc.push(elements::Paragraph::new(format!(
            "    make: {}",
            o.car.make
        )));
        doc.push(elements::Paragraph::new(format!(
            "    model: {}",
            o.car.model
        )));
        doc.push(elements::Paragraph::new(format!(
            "    year: {}",
            o.car.year
        )));
    });
    doc.push(elements::Break::new(1));
    doc.push(elements::StyledElement::new(
        elements::Paragraph::new("Total price:"),
        style::Effect::Bold,
    ));
    doc.push(elements::Paragraph::new(
        data.into_iter()
            .map(|o| o.service.price)
            .reduce(|sum, p| sum + p)
            .unwrap()
            .to_string(),
    ));
    let mut buffer = vec![0; 1 << 24];
    doc.render(buffer.as_mut_slice()).unwrap();
    buffer
}
