use async_trait::async_trait;

use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::domain::name::SandwichName;
use crate::internal::domain::sandwich::{add_sandwich, Sandwich};
use crate::internal::domain::sandwich_id::SandwichId;
use crate::internal::domain::sandwich_repository::{RepoAddError, RepoFindError, SandwichRepository};

#[async_trait]
pub trait SandwichHandler {
    async fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError>;
    async fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError>;
}

#[derive(Debug)]
pub enum CreateError {
    DomainViolation(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum GetError {
    NotFound,
    Unknown(String),
}


#[derive(Clone, Copy)]
pub struct Service<T> {
    sandwich_store: T,
}

impl<T: SandwichRepository> Service<T> {
    pub fn new(sandwich_store: T) -> Self {
        Self { sandwich_store }
    }
}

#[async_trait]
impl<T: SandwichRepository + Sync + Send> SandwichHandler for Service<T> {
    async fn handle_get_sandwich(&self, query: GetSandwich) -> Result<Sandwich, GetError> {
        let result = self.sandwich_store.find_sandwich(SandwichId::new(*query.id())).await;

        match result {
            Ok(sandwich) => Ok(sandwich),
            Err(e) => match e {
                RepoFindError::NotFound => Err(GetError::NotFound),
                RepoFindError::Unknown(e) => Err(GetError::Unknown(e))
            }
        }
    }

    async fn handle_create_sandwich(&self, command: CreateSandwich) -> Result<(), CreateError> {
        let result = add_sandwich(SandwichName::new(command.name().to_string()));

        match result {
            Ok(add_sandwich_change) => {
                match self.sandwich_store.add_sandwich(add_sandwich_change).await {
                    Ok(_) => Ok(()),
                    Err(RepoAddError::Unknown(e)) => Err(CreateError::Unknown(e))
                }
            }
            Err(e) => Err(CreateError::DomainViolation(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::domain::add_sandwich_change::AddSandwichChange;
    use super::*;

    const SANDWICH_ID: i64 = 1;
    const SANDWICH_NAME: &str = "Hot dog";

    struct SandwichStoreMock {}

    #[async_trait]
    impl SandwichRepository for SandwichStoreMock {
        async fn find_sandwich(&self, _sandwich_id: SandwichId) -> Result<Sandwich, RepoFindError> {
            Ok(Sandwich::new(
                SandwichId::new(SANDWICH_ID),
                SandwichName::new(String::from(SANDWICH_NAME)),
            ))
        }

        async fn add_sandwich(&self, _change: AddSandwichChange) -> Result<(), RepoAddError> {
            Ok(())
        }
    }

    #[actix_rt::test]
    async fn should_create_the_expected_sandwich() {
        let store_mock = SandwichStoreMock {};
        let sandwich_service = Service::new(store_mock);

        let create_sandwich_command = CreateSandwich::new(String::from(SANDWICH_NAME));

        assert_eq!(sandwich_service.handle_create_sandwich(create_sandwich_command).await.is_err(), false);
    }

    #[actix_rt::test]
    async fn should_get_the_expected_sandwich() {
        let store_mock = SandwichStoreMock {};
        let sandwich_service = Service::new(store_mock);
        let get_sandwich_query = GetSandwich::new(123);
        let expected_sandwich = Sandwich::new(
            SandwichId::new(SANDWICH_ID),
            SandwichName::new(String::from(SANDWICH_NAME)),
        );

        match sandwich_service.handle_get_sandwich(get_sandwich_query).await {
            Ok(actual_sandwich) => assert_eq!(actual_sandwich, expected_sandwich),
            Err(_) => unreachable!()
        }
    }
}



