use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DbError(String),

    #[error("Authentication error")]
    AuthError,

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Error::AuthError => HttpResponse::Unauthorized().body("Authentication error"),
            Error::JwtError(e) => HttpResponse::Unauthorized().body(e.to_string()),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::DbError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
