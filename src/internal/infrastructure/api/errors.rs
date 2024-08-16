use std::fmt;
use std::fmt::Display;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BadRequest(err)
            | ApiError::InternalServerError(err) => writeln!(f, "{},", err),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json(error)
            },
            ApiError::InternalServerError(error) => {
                HttpResponse::InternalServerError().json(error)
            }
        }
    }
}