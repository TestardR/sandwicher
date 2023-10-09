use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SandwichType {
    Meat,
    Fish,
    Veggie,
}
