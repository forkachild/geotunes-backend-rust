use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub provider: String,
    pub token: String,
}