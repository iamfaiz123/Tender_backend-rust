use std::env;
use actix_web::web;
use dotenv::dotenv;
use actix_web::{dev::Server, App, HttpServer,};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;


pub fn establish_connection() ->Result<Pool<ConnectionManager<PgConnection>>,anyhow::Error> {
    // Initilizing env file
    dotenv().ok();
    // Making connections with database.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Accuring connection manager to pass in Pool
    let manager:ConnectionManager<_>= ConnectionManager::<diesel::pg::PgConnection>::new(database_url) ;
    // The puropose of db_pool is to facilitate asyn programming
    let db_pool = diesel::r2d2::Builder::new().build(manager)?;
    Ok(db_pool)
}

pub fn spawn_server()->Result<Server,anyhow::Error>{
    // Geting Database connection
    let db_pool = establish_connection()?;
    // Setting state for easy access to different application states on different threads.
    let pool = actix_web::web::Data::new(db_pool) ;
    // The purpose of 'move' is to move our app state into server.
    let server = HttpServer::new(move || {
        App::new()
        .configure(super::open_api::openapi_config)
        .service(
            web::scope("/api").configure(crate::api::v1::v1_config)
        )
        .app_data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run() ;
    Ok(server)
}