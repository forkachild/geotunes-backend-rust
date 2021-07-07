#[derive(sqlx::FromRow)]
pub struct SongLink {
    pub id: String,
    pub song_id: String,
    pub provider: String,
    pub uri: String,
}