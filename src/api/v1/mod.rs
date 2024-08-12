use actix_web::{web, HttpResponse};
mod users;
use crate::api::v1::users::routes::*;

fn v1_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
        .service(signup)

         
    );
}