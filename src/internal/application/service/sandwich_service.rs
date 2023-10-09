use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_repository::{CreateError, GetError};
use crate::internal::domain::sandwich_type::SandwichType;

pub trait SandwichHandler {
    fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError>;
    fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError>;
}

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl SandwichHandler for Service {
    fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError> {
        let sandwich = Sandwich::new(
            query.id().to_string(),
            "hot-dog".to_string(),
            vec!["Wurst".to_string(), "Ketchup".to_string()],
            SandwichType::Meat,
        ).unwrap();

        Ok(sandwich)
    }

    fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError> {
        // let ingredients = command.ingredients().iter().map(|item| item.to_string()).collect::<Vec<String>>();
        // let sandwich = Sandwich::new(
        //     String::from(""),
        //     command.name().to_string(),
        //     ingredients,
        //     command.sandwich_type().clone(),
        // ).map_err(|e| CreateError::InvalidData(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_shared::utils::shared::{SANDWICH_NAME, SANDWICH_TYPE, string_vec_to_vec_str, stub_ingredients};
    use super::*;

    #[test]
    fn should_create_the_expected_sandwich() {
        let sandwich_service = Service::new();
        let ingredients = &stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);
        let create_sandwich_command = CreateSandwich::new(SANDWICH_NAME, &ingredients, &SANDWICH_TYPE);

        assert_eq!(sandwich_service.handle_create_sandwich(create_sandwich_command).is_err(), false);
    }

    #[test]
    fn should_get_the_expected_sandwich() {
        let sandwich_service = Service::new();
        let get_sandwich_query = GetSandwich::new("123");

        match sandwich_service.handle_get_sandwich(get_sandwich_query) {
            Ok(s) => assert_eq!(s.id().value().clone().unwrap(), "123".to_string()),
            Err(_) => unreachable!()
        }
    }
}



