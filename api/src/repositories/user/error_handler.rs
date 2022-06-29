use actix_web::error::ResponseError as ActixResponseError;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use mongodb::error::Error as MongoErr;
use std::{error::Error, fmt};
use uuid::Error as UuidError;

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    UserAlreadyExist,
    TokenError,
    WrongCredentials,
    MongoError(MongoErr),
    IdError(UuidError),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserError::UserNotFound => write!(f, "No user found with this email"),
            UserError::UserAlreadyExist => {
                write!(f, "A user with this mail is already in the database")
            }
            UserError::WrongCredentials => {
                write!(f, "No user found with this email and password")
            }
            UserError::TokenError => {
                write!(f, "Error while generating token")
            }
            UserError::MongoError(_) => write!(f, "Error from mongo"),
            UserError::IdError(_) => write!(f, "Error from uuid"),
        }
    }
}

impl Error for UserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UserError::UserNotFound => None,
            UserError::UserAlreadyExist => None,
            UserError::WrongCredentials => None,
            UserError::TokenError => None,
            UserError::MongoError(ref e) => Some(e),
            UserError::IdError(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `MongoError` to `UserError`.
impl From<MongoErr> for UserError {
    fn from(err: MongoErr) -> UserError {
        UserError::MongoError(err)
    }
}

// Implement the conversion from `UuidError` to `UserError`.
impl From<UuidError> for UserError {
    fn from(err: UuidError) -> UserError {
        UserError::IdError(err)
    }
}

// Implement the convertion to Actix `ResponseError`
impl ActixResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserAlreadyExist => StatusCode::BAD_REQUEST,
            UserError::WrongCredentials => StatusCode::BAD_REQUEST,
            UserError::TokenError => StatusCode::BAD_REQUEST,
            UserError::MongoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::IdError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
