use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::ManagedError;
use crate::types::ManagedResult;
use crate::utils::jwt;

pub struct Auth {
    pub user_id: Uuid,
}

impl Auth {
    fn extract_user_id(req: &HttpRequest) -> ManagedResult<Auth> {
        let config = req.app_data::<Data<AppConfig>>()
            .ok_or(ManagedError::Unknown)?;

        let token = req.headers().get("Authorization")
            .ok_or(ManagedError::AuthTokenMissing)?;

        let token = token.to_str()
            .map_err(|_| ManagedError::AuthTokenMissing)?;

        let user_id = jwt::decode(config.get_ref(), token)?;
        Ok(Auth { user_id })
    }
}

impl FromRequest for Auth {
    type Config = ();
    type Error = ManagedError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Self::extract_user_id(req))
    }
}