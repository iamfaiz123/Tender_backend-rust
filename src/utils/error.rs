#[allow(unused)]
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]

pub enum ServerError {
    // The from macro automatically converts specified errors into out ServerError
    #[error("database error")]
    DieselDatabaseError(#[from] diesel::ConnectionError),
    #[error("failed to execute Query")]
    QueryError(#[from] diesel::result::Error),
    #[error("failed to get pool")]
    PoolError(#[from] r2d2::Error),
    #[error("spawning task error")]
    ThreadError(#[from] actix_web::rt::task::JoinError),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::DieselDatabaseError(_) => HttpResponse::InternalServerError()
                .json("Internal Server Error: Database connection failed"),
            ServerError::QueryError(_) => HttpResponse::InternalServerError()
                .json("Internal Server Error: Query execution failed"),
            _ => {
                unimplemented!()
            }
        }
    }
}
