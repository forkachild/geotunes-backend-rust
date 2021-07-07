use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct DBNearbySong {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub provider: String,
    pub uri: String,
    pub play_count: i64,
}