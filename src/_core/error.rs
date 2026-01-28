use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DbError(String),

    #[error("Authentication error")]
    AuthError,

    #[error("Permisson denied")]
    PermissionError,

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("ResourceNot found: {0}")]
    ResourceNotFound(String),

    #[error("Failed to create resource: {0}")]
    CreateResourceError(String),

    #[error("Failed to update resource: {0}")]
    UpdateResourceError(String),

    #[error("Failed to delete resource: {0}")]
    DeleteResourceError(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError(e) => {
                HttpResponse::InternalServerError().body(e.to_string())
            }
            Error::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            Error::AuthError => HttpResponse::Unauthorized().body("Authentication error"),
            Error::JwtError(e) => HttpResponse::Unauthorized().body(e.to_string()),
            Error::PermissionError => HttpResponse::Forbidden().body("Permission denied"),
            Error::ResourceNotFound(e) => HttpResponse::BadRequest().body(e.to_string()),
            Error::CreateResourceError(e) => HttpResponse::BadRequest().body(e.to_string()),
            Error::UpdateResourceError(e) => HttpResponse::BadRequest().body(e.to_string()),
            Error::DeleteResourceError(e) => HttpResponse::BadRequest().body(e.to_string()),
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
