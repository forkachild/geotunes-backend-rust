use serde::Deserialize;
use uuid::Uuid;

use crate::routes::model::request::location::Location;

#[derive(Debug, Deserialize)]
pub struct AddLocalSong {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub provider: String,
    pub location: Location,
    pub world_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AddExistingSong {
    pub song_id: Uuid,
    pub location: Location,
    pub world_id: Option<Uuid>,
}