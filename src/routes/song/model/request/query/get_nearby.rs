use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GetNearby {
    pub lat: f64,
    pub lon: f64,
    pub provider: String,
    pub world_id: Option<Uuid>,
}