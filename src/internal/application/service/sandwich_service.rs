use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;
use crate::internal::domain::sandwich_repository::{RepoCreateError, RepoGetError, Repository};

pub trait SandwichHandler {
    fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError>;
    fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError>;
}

#[derive(Debug)]
pub enum CreateError {
    Unknown(String),
    InvalidData(String),
}

#[derive(Debug)]
pub enum GetError {
    Unknown(String),
    NotFound,
}

#[derive(Clone)]
pub struct Service<'a> {
    sandwich_store: &'a dyn Repository,
}

impl<'a> Service<'a> {
    pub fn new(sandwich_store: &'a dyn Repository) -> Self {
        Self { sandwich_store }
    }
}

impl SandwichHandler for Service {
    fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError> {
        self.sandwich_store.find_one(SandwichId::try_from(query.id().to_string())?)
            .map(|sandwich| {
                Ok(sandwich)
            })
            .map_err(|e| match e {
                RepoGetError::NotFound => GetError::NotFound,
                e => GetError::Unknown(e.to_string()),
            })?
    }

    fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError> {
        let ingredients = command.ingredients().iter().map(|item| item.to_string()).collect::<Vec<String>>();

        let sandwich = Sandwich::new(
            "".to_string(),
            command.name().to_string(),
            ingredients,
            command.sandwich_type().clone(),
        )?;

        let result = self.sandwich_store.create(sandwich);

        match result {
            Ok(_) => Ok(()),
            RepoCreateError::Unknown => CreateError::Unknown
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::test_shared::utils::shared::{SANDWICH_NAME, SANDWICH_TYPE, string_vec_to_vec_str, stub_ingredients};
//     use super::*;
//
//     #[test]
//     fn should_create_the_expected_sandwich() {
//         let sandwich_service = Service::new();
//         let ingredients = &stub_ingredients();
//         let ingredients = string_vec_to_vec_str(&ingredients);
//         let create_sandwich_command = CreateSandwich::new(SANDWICH_NAME, &ingredients, &SANDWICH_TYPE);
//
//         assert_eq!(sandwich_service.handle_create_sandwich(create_sandwich_command).is_err(), false);
//     }
//
//     #[test]
//     fn should_get_the_expected_sandwich() {
//         let sandwich_service = Service::new();
//         let get_sandwich_query = GetSandwich::new("123");
//
//         match sandwich_service.handle_get_sandwich(get_sandwich_query) {
//             Ok(s) => assert_eq!(s.id().value().clone().unwrap(), "123".to_string()),
//             Err(_) => unreachable!()
//         }
//     }
// }



