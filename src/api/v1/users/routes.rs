use crate::api::v1::users::forms::SignupForm;
use crate::api::v1::users::forms::SigninForm;
use crate::api::v1::users::models;
use crate::api::v1::users::models::*;
use crate::api::v1::users::response::UserSignupResponse;
use crate::api::v1::users::response::UserSigninResponse;
use crate::utils::error::ServerError;
use actix_web::rt::task::spawn_blocking;
use actix_web::HttpResponse;
use actix_web::{post, web};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel::prelude::*;
use crate::schema::{users, user_roles};


#[utoipa::path(
    post,
    path="api/v1/signup",
    request_body = SignupForm ,
    responses(
        ( 
            
            status = 201 ,
            description = "user profile is created" ,
            example = json!({"user_id":"9def5c3f-94db-48f3-b3aa-23b675fef720" , "role" : ["CLIENT" , "BUILDER"]})

        ),
        (
            status = 500 ,
            description = "internal server error"
  
        )
    )
)]

#[post("/signup")]
/// api to crate a user signup request 
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

// Define the signin API endpoint using the utoipa library
#[utoipa::path(
    post, // Specify the HTTP method for the endpoint
    path="api/v1/signin", // Specify the path for the endpoint
    request_body = SigninForm, // Specify the request body type
    responses( // Define the possible responses for the endpoint
        ( 
            status = 200, // Successful signin response
            description = "user signed in successfully", // Description of the response
            example = json!({"user_id":"9def5c3f-94db-48f3-b3aa-23b675fef720" , "role" : ["CLIENT" , "BUILDER"]}) // Example response body
        ),
        (
            status = 401, // Unauthorized response
            description = "invalid email or password" // Description of the response
        ),
        (
            status = 500, // Internal server error response
            description = "internal server error" // Description of the response
        )
    )
)]
// Define the signin function as an async function
#[post("/signin")] // Specify the route for the function
/// api to signin a user // Documentation comment for the function
pub async fn signin(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<SigninForm>)
    // Request body containing the signin form data
     -> Result<HttpResponse, ServerError> { // Return type for the function
    // Execute the signin logic in a blocking manner using spawn_blocking
    let resp: UserSigninResponse = spawn_blocking(move || {
        // Get a connection from the pool
        let mut conn = conn.get()?;
        
        // Start a database transaction
        let resp: UserSigninResponse = conn.build_transaction().run(|mut conn| {
            // Extract the email and password from the request body
            let SigninForm { email, password } = req.into_inner();
            
            // Query the users table to find a user with the given email
            let user = users::table
                .filter(users::email.eq(email)) // Filter the users table by email
                .first::<models::User>(conn).optional()?; // Get the first matching user
            
            // Check if a user was found
            if let Some(user) = user {
                // Check if the password matches
                if user.password == password {
                    // Query the user_roles table to find the roles for the user
                    let user_roles = user_roles::table
                        .filter(user_roles::user_id.eq(user.id)) // Filter the user_roles table by user ID
                        .load::<UserRoleXref>(conn)?; // Load the matching roles
                    
                    // Convert the roles to a vector of strings
                    let user_roles_vec: Vec<String> = user_roles
                        .into_iter()
                        .map(|x| x.role.to_string()) // Map each role to a string
                        .collect(); // Collect the results into a vector
                    
                    // Create a UserSigninResponse with the user ID and roles
                    let resp = UserSigninResponse {
                        user_id: user.id,
                        role: user_roles_vec,
                    };
                    
                    // Return the response as Ok
                    Result::<UserSigninResponse, ServerError>::Ok(resp)
                } else {
                    // Return an error if the password does not match
                    Result::<UserSigninResponse, ServerError>::Err(ServerError::Unauthorized)
                }
            } else {
                // Return an error if no user was found
                Result::<UserSigninResponse, ServerError>::Err(ServerError::Unauthorized)
            }
        })?;
        
        // Return the response as Ok
        Result::<UserSigninResponse, ServerError>::Ok(resp)
    })
    .await??; // Await the result of the blocking execution
    
    // Return the response as a JSON response
    return Ok(HttpResponse::Ok().json(resp))
}
