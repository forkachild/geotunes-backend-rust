use std::ops::Add;
use std::time::{Duration, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::config::AppConfig;
use crate::types::ManagedResult;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    exp: usize,
}

const SIX_MONTHS: u64 = 60 * 60 * 24 * 30 * 6;

pub fn encode(config: &AppConfig, user_id: &Uuid) -> ManagedResult<String> {
    let current_time = std::time::SystemTime::now();
    let duration = current_time.duration_since(UNIX_EPOCH)?;
    let duration = duration.add(Duration::from_secs(SIX_MONTHS));
    let expiry_time = duration.as_millis() as usize;

    let claims = Claims {
        user_id: user_id.to_owned(),
        exp: expiry_time,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_ref());
    Ok(jsonwebtoken::encode(&header, &claims, &encoding_key)?)
}

pub fn decode(config: &AppConfig, token: &str) -> ManagedResult<Uuid> {
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
    let validation = Validation::default();
    Ok(jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?.claims.user_id)
}