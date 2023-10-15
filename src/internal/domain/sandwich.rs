use std::fmt::Error;
use crate::internal::domain::add_sandwich_change::{AddSandwichChange, new_add_sandwich_change};
use crate::internal::domain::errors::DomainViolation;
use crate::internal::domain::name::SandwichName;
use crate::internal::domain::sandwich_id::SandwichId;

#[derive(Debug, PartialEq)]
pub struct Sandwich {
    id: SandwichId,
    name: SandwichName,
}

impl Sandwich {
    pub fn new(id: SandwichId, name: SandwichName) -> Sandwich {
        Self {
            id,
            name,
        }
    }

    pub fn id(&self) -> &SandwichId {
        &self.id
    }

    pub fn name(&self) -> &SandwichName {
        &self.name
    }
}

pub fn add_sandwich(
    name: SandwichName,
) -> Result<AddSandwichChange, DomainViolation> {
    let mut violations = vec![];

    if name.value() == "" {
        violations.push("name is required".to_string());
    }

    if violations.len() > 0 {
        return Err(DomainViolation::new(violations.join(", ")));
    }

    Ok(new_add_sandwich_change(name))
}

mod tests {
    use super::*;

    const SANDWICH_ID: i64 = 1;
    const SANDWICH_NAME: &str = "Hot dog";

    #[test]
    fn should_create_the_expected_sandwich() {
        let hot_dog = Sandwich::new(
            SandwichId::new(SANDWICH_ID),
            SandwichName::new(SANDWICH_NAME.to_string())
        );

        assert_eq!(*hot_dog.id().value(), SANDWICH_ID);
        assert_eq!(hot_dog.name().value(), SANDWICH_NAME);
    }
}