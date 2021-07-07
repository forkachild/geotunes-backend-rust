use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetWorlds {
    pub name: Option<String>,
    pub skip: Option<u32>,
    pub limit: Option<u32>,
}