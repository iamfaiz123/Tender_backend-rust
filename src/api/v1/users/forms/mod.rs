use super::models::Role;

#[derive(serde::Deserialize)]
pub struct SignupForm{
    pub email:String ,
    pub first_name:String,
    pub last_name:String,
    pub password:String,
    pub roles:Vec<Role>
}