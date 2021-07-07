use std::time::SystemTimeError;

use actix_http::Response;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::Body;
use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError};
use actix_web::http::StatusCode;
use serde_json::json;
use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug)]
pub enum ManagedError {
    Unknown,
    EnvironmentVariable,
    Database,
    BadRoute(String, String),
    AuthTokenMissing,
    AuthTokenInvalid,
    Query(QueryPayloadError),
    Json(JsonPayloadError),
    Path(PathError),

    AlreadyExists,
    DoesNotExist,
}

impl ManagedError {
    pub fn code(&self) -> u32 {
        match self {
            ManagedError::Unknown => 1,
            ManagedError::EnvironmentVariable => 2,
            ManagedError::Database => 3,
            ManagedError::BadRoute(_, _) => 4,
            ManagedError::AuthTokenMissing => 5,
            ManagedError::AuthTokenInvalid => 6,
            ManagedError::Query(_) => 7,
            ManagedError::Json(_) => 8,
            ManagedError::Path(_) => 9,

            ManagedError::AlreadyExists => 10,
            ManagedError::DoesNotExist => 11,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ManagedError::Unknown => "Unknown error".to_owned(),
            ManagedError::EnvironmentVariable => "Environment variable not set".to_owned(),
            ManagedError::Database => "Database error".to_owned(),
            ManagedError::BadRoute(method, route) => format!("[{} {}] does not point to a resource", method, route),
            ManagedError::AuthTokenMissing => "Authorization token missing in header".to_owned(),
            ManagedError::AuthTokenInvalid => "Authorization token invalid".to_owned(),
            ManagedError::Query(err) => format!("{:?}", err),
            ManagedError::Json(err) => format!("{:?}", err),
            ManagedError::Path(err) => format!("{:?}", err),

            ManagedError::AlreadyExists => "Already exists".to_owned(),
            ManagedError::DoesNotExist => "Does not exist".to_owned()
        }
    }
}

impl core::fmt::Display for ManagedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error: {}", self.message())
    }
}

impl From<std::env::VarError> for ManagedError {
    fn from(_err: std::env::VarError) -> Self {
        #[cfg(debug_assertions)]
        println!("Error: {:?}", _err);

        Self::EnvironmentVariable
    }
}

impl From<sqlx::Error> for ManagedError {
    fn from(_err: sqlx::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("Error: {:?}", _err);

        Self::Database
    }
}

impl From<jsonwebtoken::errors::Error> for ManagedError {
    fn from(_err: jsonwebtoken::errors::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("Error: {:?}", _err);

        Self::AuthTokenInvalid
    }
}

impl From<std::io::Error> for ManagedError {
    fn from(_err: std::io::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("Error: {:?}", _err);

        Self::Unknown
    }
}

impl From<SystemTimeError> for ManagedError {
    fn from(_err: SystemTimeError) -> Self {
        #[cfg(debug_assertions)]
        println!("Error {:?}", _err);

        Self::Unknown
    }
}

impl From<ParseIntError> for ManagedError {
    fn from(_err: ParseIntError) -> Self {
        #[cfg(debug_assertions)]
        println!("Error {:?}", _err);

        Self::Unknown
    }
}

impl From<ParseFloatError> for ManagedError {
    fn from(_err: ParseFloatError) -> Self {
        #[cfg(debug_assertions)]
        println!("Error {:?}", _err);

        Self::Unknown
    }
}

impl ResponseError for ManagedError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> Response<Body> {
        HttpResponse::Ok().json(json!({
            "code": self.code(),
            "message": self.message()
        })).into()
    }
}