use crate::types::ManagedResult;

pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub database_max_conn: u32,
    pub search_radius: f64,
    pub jwt_secret: String,
    pub default_skip: u32,
    pub default_limit: u32,
}

impl AppConfig {
    pub fn from_env() -> ManagedResult<Self> {
        let port = std::env::var("PORT")?.parse::<u16>()?;
        let database_url = std::env::var("DATABASE_URL")?;
        let database_max_conn = std::env::var("DATABASE_MAX_CONN")?.parse::<u32>()?;
        let search_radius = std::env::var("SEARCH_RADIUS")?.parse::<f64>()?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let default_skip = std::env::var("DEFAULT_SKIP")?.parse::<u32>()?;
        let default_limit = std::env::var("DEFAULT_LIMIT")?.parse::<u32>()?;

        Ok(
            Self {
                port,
                database_url,
                database_max_conn,
                search_radius,
                jwt_secret,
                default_skip,
                default_limit
            }
        )
    }
}