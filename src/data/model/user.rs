#[derive(sqlx::FromRow)]
pub struct User {
    pub id: String,
}

#[derive(sqlx::FromRow)]
pub struct UserDetails {
    pub id: String,
    pub user_id: String,
    pub email: String,
}

#[derive(sqlx::FromRow)]
pub struct UserAuthProvider {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub token: String,
}

#[derive(sqlx::FromRow)]
pub struct UserAuthSession {
    pub id: String,
    pub user_id: String,
    pub auth_provider_id: String,
    pub access_token: String,
}