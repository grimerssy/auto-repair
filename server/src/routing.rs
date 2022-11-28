use crate::api::*;
use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(users::signup)
                    .service(users::login),
            )
            .service(
                web::scope("/cars")
                    .service(cars::create)
                    .service(cars::create_for_self)
                    .service(cars::get_all)
                    .service(cars::get_for_self)
                    .service(cars::get_by_vin)
                    .service(cars::get_by_vin_for_self)
                    .service(cars::update_by_vin)
                    .service(cars::update_by_vin_for_self)
                    .service(cars::delete_by_vin)
                    .service(cars::delete_by_vin_for_self),
            )
            .service(
                web::scope("/contacts")
                    .service(contacts::get_all)
                    .service(contacts::get_self)
                    .service(contacts::get_by_id)
                    .service(contacts::update_self)
                    .service(contacts::update_by_id)
                    .service(contacts::delete_by_id),
            )
            .service(
                web::scope("/orders")
                    .service(orders::get_available_time)
                    .service(orders::create)
                    .service(orders::get_all)
                    .service(orders::get_all_for_self)
                    .service(orders::get_by_id)
                    .service(orders::get_by_service_id)
                    .service(orders::update_by_id)
                    .service(orders::delete_by_id)
                    .service(orders::delete_by_ids)
                    .service(orders::get_receipt)
                    .service(orders::get_receipt_for_self),
            )
            .service(
                web::scope("/services")
                    .service(services::create)
                    .service(services::get_all)
                    .service(services::get_by_id)
                    .service(services::get_by_title)
                    .service(services::delete_by_id)
                    .service(services::update_by_id)
                    .service(services::add_for_worker)
                    .service(services::get_for_worker)
                    .service(services::remove_for_worker),
            )
            .service(
                web::scope("/workers")
                    .service(workers::create)
                    .service(workers::get_all)
                    .service(workers::get_by_id)
                    .service(workers::update_by_id)
                    .service(workers::delete_by_id),
            )
            .service(
                web::scope("/reports")
                    .service(reports::get_most_profitable_services_for_month)
                    .service(reports::get_most_valuable_clients_for_month)
                    .service(reports::get_most_frequent_cars_for_month)
                    .service(reports::get_total_work_hours_for_month)
                    .service(reports::get_pdf_report),
            )
            .service(web::scope("/sql").service(sql::do_sql)),
    );
}
