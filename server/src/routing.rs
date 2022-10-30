use crate::api::*;
use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/sql").service(sql::do_sql))
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
                    .service(orders::create)
                    .service(orders::get_all)
                    .service(orders::get_by_id)
                    .service(orders::get_by_service_id)
                    .service(orders::update_by_id)
                    .service(orders::delete_by_id),
            )
            .service(
                web::scope("/services")
                    .service(services::create)
                    .service(services::get_all)
                    .service(services::get_by_id)
                    .service(services::delete_by_id)
                    .service(services::update_by_id),
            )
            .service(
                web::scope("/auth")
                    .service(users::signup)
                    .service(users::login),
            ),
    );
}
