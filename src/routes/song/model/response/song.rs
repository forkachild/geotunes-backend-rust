use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct NearbySong {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub provider: String,
    pub uri: String,
    pub play_count: i64,
}

impl NearbySong {
    pub fn new(
        id: Uuid,
        title: String,
        artist: String,
        provider: String,
        uri: String,
        play_count: i64,
    ) -> Self {
        Self {
            id,
            title,
            artist,
            provider,
            uri,
            play_count,
        }
    }
}