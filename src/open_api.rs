use actix_web::web;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::api::v1::users::forms::*;
use crate::api::v1::users::routes::*;
#[derive(utoipa::OpenApi)]
#[openapi(
    info(title="Tender-website Serivce API DOCS" , version = "1.0" , description = " default "),
    paths(
        signup,signin
    ),
    components(schemas(SignupForm,SigninForm)),
    tags((name = "static content" , description = "Tender website service apis"))
)]

struct TenderServiceApi;

pub(crate) fn openapi_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-doc/openapi.json", TenderServiceApi::openapi()),
    );
}
