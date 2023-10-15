use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::internal::domain::name::SandwichName;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SandwichEntity {
    id: i64,
    name: String,
}

impl SandwichEntity {
    pub fn from_entity_to_domain_model(&self) -> Sandwich {
        let entity_id = &self.id;
        let entity_name = &self.name;
        let id = SandwichId::new(*entity_id);
        let name = SandwichName::new(entity_name.to_string());

        Sandwich::new(id, name)
    }
}

