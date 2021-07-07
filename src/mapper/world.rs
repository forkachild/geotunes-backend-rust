use crate::data::world::models::DBWorld;
use crate::routes::world::model::response::world::WorldResponse;

impl From<DBWorld> for WorldResponse {
    fn from(value: DBWorld) -> Self {
        Self::new(value.id, value.name)
    }
}