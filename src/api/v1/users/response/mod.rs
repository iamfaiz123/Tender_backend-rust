use crate::api::v1::users::models::Role;

#[derive(serde::Serialize)]
pub struct UserSignupResponse {
    pub user_id: uuid::Uuid,
    pub user_roles: Vec<String>,
}
