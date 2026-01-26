use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Db(String),

    #[error("Authentication error")]
    AuthError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Error::AuthError => HttpResponse::Unauthorized().body("Authentication error"),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
