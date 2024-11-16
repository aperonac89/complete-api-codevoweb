use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct Response {
    pub status: &'static str, 
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceedMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionsDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password are wrong".to_string(),
            ErrorMessage::EmailExist => "Email already exists in the database".to_string(),
            ErrorMessage::UserNoLongerExists => {
                "User belonging to this token no longer exists".to_string()
            },
            ErrorMessage::EmptyPassword => "Password should not be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid passowrd hash format".to_string(), 
            ErrorMessage::ExceedMaxPasswordLength(max_length) => {
                format!("Password should not exceed {} characters", max_length)
            },
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionsDenied => "Not allowed to perform this action".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String, 
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status:u16) -> Self {
        HttpError{
            message: message.into(),
            status: status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500
        }
    }
    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 404
        }
    }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json( Response {
                status: "fail",
                message: self.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json( Response {
                status: "fail",
                message: self.message.into(),
            }),
            404 => HttpResponse::NotFound().json( Response {
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json( Response {
                status: "fail",
                message: self.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json( Response {
                status: "error",
                message: self.message.into(),
            }),
            _ => {
                eprintln!(
                    "warning: missing mattern match. Converted status code {} to 500",
                    self.status
                )
            }
        }
    }
}   

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "HttpError, message {}, status{}",
            self.message,
            self.status
        )
    }
}

impl std::error::Error for HttpError{}

impl RepsonseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}