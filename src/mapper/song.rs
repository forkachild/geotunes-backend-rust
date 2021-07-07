use crate::data::song::models::DBNearbySong;
use crate::routes::song::model::response::song::NearbySong;

impl From<DBNearbySong> for NearbySong {
    fn from(value: DBNearbySong) -> Self {
        Self::new(
            value.id,
            value.title,
            value.artist,
            value.provider,
            value.uri,
            value.play_count,
        )
    }
}