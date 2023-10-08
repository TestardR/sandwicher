use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::domain::sandwich::Sandwich;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String)
}

#[derive(Debug)]
pub enum GetError {
    Unknown(String),
    NotFound,
}
pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<Sandwich, CreateError> {
        let ingredients = command.ingredients().iter().map(|item| item.to_string()).collect::<Vec<String>>();
        let sandwich = Sandwich::new(
            String::from(""),
            command.name().to_string(),
            ingredients,
            command.sandwich_type().clone()
        ).map_err(|e| CreateError::InvalidData(e))?;

        Ok(sandwich)
    }

    fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError> {
        let ingredients = query.ingredients().iter().map(|item| item.to_string()).collect::<Vec<String>>();
        let sandwich = Sandwich::new(
            query.id().to_string(),
            query.name().to_string(),
            ingredients,
            query.sandwich_type().clone()
        ).unwrap();

        Ok(sandwich)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_shared::utils::shared::{assert_on_sandwich, SANDWICH_NAME, SANDWICH_TYPE, string_vec_to_vec_str, stub_ingredients, stub_sandwich};
    use super::*;

    #[test]
    fn should_create_the_expected_sandwich() {
        let sandwich_service = Service::new();
        let ingredients = &stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);
        let create_sandwich_command = CreateSandwich::new(SANDWICH_NAME, &ingredients, &SANDWICH_TYPE);

        let sandwich = sandwich_service.handle_create_sandwich(create_sandwich_command).unwrap();

        assert_on_sandwich(sandwich, &stub_sandwich(false), false);
    }

    #[test]
    fn should_get_the_expected_sandwich() {
        let sandwich_service = Service::new();
        let ingredients = &stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);
        let get_sandwich_query = GetSandwich::new("", SANDWICH_NAME, &ingredients, &SANDWICH_TYPE);

        match sandwich_service.handle_get_sandwich(get_sandwich_query) {
            Ok(s) => assert_on_sandwich(stub_sandwich(false), &s, false),
            Err(_) => unreachable!()
        }
    }
}



