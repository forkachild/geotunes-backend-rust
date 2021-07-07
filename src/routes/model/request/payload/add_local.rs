use serde::Deserialize;

use crate::routes::model::request::location::Location;

#[derive(Debug, Deserialize)]
pub struct AddLocal {
    pub provider: String,
    pub uri: String,
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExisting {
    pub song_id: String,
    pub station_id: String,
    pub location: Location,
}