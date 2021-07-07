use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}