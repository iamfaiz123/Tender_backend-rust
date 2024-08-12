use crate::api::v1::users::forms::SignupForm;
use crate::api::v1::users::models::*;
use crate::api::v1::users::response::UserSignupResponse;
use crate::utils::error::ServerError;
use actix_web::rt::task::spawn_blocking;
use actix_web::HttpResponse;
use actix_web::{post, web};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[post("/signup")]
pub async fn signup(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<SignupForm>,
) -> Result<HttpResponse, ServerError> {
    let resp:UserSignupResponse = spawn_blocking(move || {
        // get connection
        let mut user_roles: Vec<String> = vec![];
        let mut conn = conn.get()?;
        let resp:UserSignupResponse = conn.build_transaction().run(|mut conn| {
            // create a new user
            let SignupForm {
                email,
                first_name,
                last_name,
                password,
                roles,
            } = req.into_inner();
            let new_user = User::new(email, password, first_name, last_name);
            // insert user
            let user_id = new_user.insert(&mut conn)?;
            let user_roles_vec: Vec<UserRoleXref> = roles
                .into_iter()
                .map(|role| {
                    user_roles.push(role.as_str().to_string());
                    UserRoleXref {
                        user_id,
                        role: role,
                    }
                })
                .collect();
            // insert user roles
            UserRoleXref::insert_user_roles(&user_roles_vec, &mut conn)?;
            let resp = UserSignupResponse {
                user_id,
                user_roles,
            };
            Result::<UserSignupResponse, ServerError>::Ok(resp)
        })?;
        Result::<UserSignupResponse, ServerError>::Ok(resp)
    })
    .await??;

    return Ok(
        HttpResponse::Created().json(resp)
    )
}
