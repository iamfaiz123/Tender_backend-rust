use crate::api::v1::users::models::Role;

#[derive(serde::Serialize)]
pub struct UserSignupResponse {
    pub user_id: uuid::Uuid,
    pub user_roles: Vec<String>,
}

use serde::{Serialize, Deserialize};

// Removed ToSchema for now.
#[derive(Serialize, Deserialize)]
pub struct UserSigninResponse {
    pub user_id: String,
    pub role: Vec<String>,
}
