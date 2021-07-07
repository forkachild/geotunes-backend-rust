use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct WorldResponse {
    pub id: Uuid,
    pub name: String,
}

impl WorldResponse {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}