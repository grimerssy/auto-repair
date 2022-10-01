use crate::api::*;
use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(web::scope("/services").service(services::get_all)));
}
