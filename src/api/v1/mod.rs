use actix_web::web;
pub mod users;
use crate::api::v1::users::routes::*;

pub fn v1_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
        .service(signup)

         
    );
}