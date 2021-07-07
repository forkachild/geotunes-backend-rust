use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWorld {
    pub name: String,
}