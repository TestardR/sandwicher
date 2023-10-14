use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_type::SandwichType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SandwichEntity {
    id: i64,
    name: String,
    ingredients: Vec<String>,
    sandwich_type: SandwichType,
}

impl From<Sandwich> for SandwichEntity {
    fn from(sandwich: Sandwich) -> Self {
        let id = match sandwich.id().value() {
            Some(id) => id,
            None => ""
        };

        let entity = SandwichEntity {
            id: id.parse::<i64>().unwrap(),
            name: sandwich.name().value().to_string(),
            ingredients: sandwich.ingredients().value().clone(),
            sandwich_type: sandwich.sandwich_type().clone(),
        };

        entity
    }
}

impl TryInto<Sandwich> for SandwichEntity {
    type Error = String;
    fn try_into(self) -> Result<Sandwich, Self::Error> {
        Sandwich::new(self.id.to_string(),
                      self.name,
                      self.ingredients,
                      self.sandwich_type,
                      )
    }
}
