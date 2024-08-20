use utoipa::{openapi::example, ToSchema};

use super::models::Role;

#[derive(serde::Deserialize,ToSchema)]
pub struct SignupForm{
    #[schema(example = "tender@gmail.com")]
    pub email:String ,
    #[schema(example = "john")]
    pub first_name:String,
    #[schema (example = "doe")]
    pub last_name:String,
    #[schema ( example = "somer!##ando232mpass!#!#word")]
    pub password:String,
    #[schema (example = "['CLIENT']")]
    pub roles:Vec<Role>
}

// Sign in form
#[derive(serde::Deserialize, ToSchema)]
pub struct SigninForm {
    #[schema(example = "tender@gmail.com")]
    pub email: String,
    #[schema(example = "somer!##ando232mpass!#!#word")]
    pub password: String,
}