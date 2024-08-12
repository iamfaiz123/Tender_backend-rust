use std::env;
use dotenv::dotenv;
use actix_web::{dev::Server, App, HttpServer,};
use diesel::{pg::PgConnection, Connection as _};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn establish_connection() ->Result<Pool<ConnectionManager<PgConnection>>,anyhow::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager:ConnectionManager<_>= ConnectionManager::<diesel::pg::PgConnection>::new(database_url) ;
    let db_pool = diesel::r2d2::Builder::new().build(manager)?;
    Ok(db_pool)
}

pub fn spawn_server()->Result<Server,anyhow::Error>{
    // get pg connection
    let db_pool = establish_connection()?;
    let pool = actix_web::web::Data::new(db_pool) ;
    let server = HttpServer::new(move || {
        App::new()
        .app_data(pool.clone())
    })
    .bind("127.0.0.1:8080")?
    .run() ;
    Ok(server)
}