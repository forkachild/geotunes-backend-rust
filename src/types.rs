use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::errors::ManagedError;

pub struct ManagedResponse<T: Serialize>(T);

impl<T: Serialize> From<T> for ManagedResponse<T> {
    fn from(data: T) -> Self {
        Self(data)
    }
}

impl<T: Serialize> Responder for ManagedResponse<T> {
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::build(StatusCode::OK).json(json!({
                "code": 0,
                "data": self.0
            }
        ))
    }
}

pub trait IntoResponseResult<T: Serialize> {
    fn into_response(self) -> ResponseResult<T>;
}

impl<T: Serialize> IntoResponseResult<T> for T {
    fn into_response(self) -> ResponseResult<T> {
        Ok(ManagedResponse(self))
    }
}

pub type ManagedResult<T> = Result<T, ManagedError>;
pub type ResponseResult<T> = ManagedResult<ManagedResponse<T>>;