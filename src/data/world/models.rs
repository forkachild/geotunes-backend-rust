use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct DBWorld {
    pub id: Uuid,
    pub name: String,
}