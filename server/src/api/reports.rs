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
    HttpRequest, HttpResponse,
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

#[get("/pdf")]
pub async fn get_pdf_report(
    req: HttpRequest,
    db_pool: Data<DbPool>,
    jwt_cfg: Data<JwtCfg>,
) -> Result<HttpResponse> {
    check_if_admin(get_claims(&req, &jwt_cfg.secret).await?)?;
    let conn = &mut retrieve_connection(db_pool).await?;
    let font_family = genpdf::fonts::from_files("./assets/fonts/Roboto", "Roboto", None).unwrap();
    let services = reports::get_most_profitable_services_for_month(conn)
        .await
        .map_err(from_diesel_error())?;
    let clients = reports::get_most_valuable_clients_for_month(conn)
        .await
        .map_err(from_diesel_error())?;
    let cars = reports::get_most_frequent_cars_for_month(conn)
        .await
        .map_err(from_diesel_error())?;
    let working_hours = reports::get_total_work_hours_for_month(conn)
        .await
        .map(|x| x.hours)
        .map_err(from_diesel_error())?;
    let mut doc = genpdf::Document::new(font_family);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(15);
    doc.set_page_decorator(decorator);
    doc.set_title("Report");
    doc.set_line_spacing(1.5);
    doc.set_font_size(14);
    doc.push(genpdf::elements::StyledElement::new(
        genpdf::elements::Paragraph::new("Most valuable clients this month:"),
        genpdf::style::Effect::Bold,
    ));
    doc.push(genpdf::elements::Break::new(1));
    let mut table = genpdf::elements::TableLayout::new(vec![1, 2, 1]);
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Phone number"))
        .element(genpdf::elements::Paragraph::new("Email"))
        .element(genpdf::elements::Paragraph::new("Order count"))
        .push()
        .unwrap();
    clients.clone().into_iter().for_each(|c| {
        table
            .row()
            .element(genpdf::elements::Paragraph::new(c.phone_number))
            .element(genpdf::elements::Paragraph::new(
                c.email.unwrap_or_else(|| "".into()),
            ))
            .element(genpdf::elements::Paragraph::new(c.order_count.to_string()))
            .push()
            .unwrap();
    });
    doc.push(table);
    doc.push(genpdf::elements::Break::new(2));
    doc.push(genpdf::elements::Paragraph::new("Additional data: "));
    doc.push(genpdf::elements::Break::new(1));
    let mut table = genpdf::elements::TableLayout::new(vec![1, 1, 1, 1, 1]);
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Phone number"))
        .element(genpdf::elements::Paragraph::new("First name"))
        .element(genpdf::elements::Paragraph::new("Middle name"))
        .element(genpdf::elements::Paragraph::new("Last name"))
        .element(genpdf::elements::Paragraph::new("Date of birth"))
        .push()
        .unwrap();
    clients.into_iter().for_each(|c| {
        table
            .row()
            .element(genpdf::elements::Paragraph::new(c.phone_number))
            .element(genpdf::elements::Paragraph::new(
                c.first_name.unwrap_or_else(|| "".into()),
            ))
            .element(genpdf::elements::Paragraph::new(
                c.middle_name.unwrap_or_else(|| "".into()),
            ))
            .element(genpdf::elements::Paragraph::new(
                c.last_name.unwrap_or_else(|| "".into()),
            ))
            .element(genpdf::elements::Paragraph::new(
                c.date_of_birth.unwrap_or_else(|| "".into()),
            ))
            .push()
            .unwrap();
    });
    doc.push(table);
    doc.push(genpdf::elements::Break::new(2));
    doc.push(genpdf::elements::StyledElement::new(
        genpdf::elements::Paragraph::new("Most frequent cars this month:"),
        genpdf::style::Effect::Bold,
    ));
    doc.push(genpdf::elements::Break::new(1));
    let mut table = genpdf::elements::TableLayout::new(vec![1, 1, 1, 1]);
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Make"))
        .element(genpdf::elements::Paragraph::new("Model"))
        .element(genpdf::elements::Paragraph::new("Year"))
        .element(genpdf::elements::Paragraph::new("Order count"))
        .push()
        .unwrap();
    cars.into_iter().for_each(|c| {
        table
            .row()
            .element(genpdf::elements::Paragraph::new(c.make))
            .element(genpdf::elements::Paragraph::new(c.model))
            .element(genpdf::elements::Paragraph::new(c.year.to_string()))
            .element(genpdf::elements::Paragraph::new(c.order_count.to_string()))
            .push()
            .unwrap();
    });
    doc.push(table);
    doc.push(genpdf::elements::PageBreak::new());
    doc.push(genpdf::elements::StyledElement::new(
        genpdf::elements::Paragraph::new("Most profitable services this month:"),
        genpdf::style::Effect::Bold,
    ));
    doc.push(genpdf::elements::Break::new(1));
    let mut table = genpdf::elements::TableLayout::new(vec![2, 1, 1, 1]);
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Title"))
        .element(genpdf::elements::Paragraph::new("Price"))
        .element(genpdf::elements::Paragraph::new("Duration"))
        .element(genpdf::elements::Paragraph::new("Order count"))
        .push()
        .unwrap();
    services.into_iter().for_each(|s| {
        table
            .row()
            .element(genpdf::elements::Paragraph::new(s.title))
            .element(genpdf::elements::Paragraph::new(s.price.to_string()))
            .element(genpdf::elements::Paragraph::new(s.duration.to_string()))
            .element(genpdf::elements::Paragraph::new(s.order_count.to_string()))
            .push()
            .unwrap();
    });
    doc.push(table);
    doc.push(genpdf::elements::Break::new(2));
    doc.push(genpdf::elements::StyledElement::new(
        genpdf::elements::Text::new("Total working hours this month:"),
        genpdf::style::Effect::Bold,
    ));
    doc.push(genpdf::elements::Paragraph::new(working_hours.to_string()));
    let mut buffer = vec![0; 1 << 24];
    doc.render(buffer.as_mut_slice()).unwrap();
    Ok(HttpResponse::Ok()
        .insert_header(("content-type", "application/pdf"))
        .body(buffer))
}
