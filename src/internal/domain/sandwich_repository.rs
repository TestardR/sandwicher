use async_trait::async_trait;
use crate::internal::domain::add_sandwich_change::AddSandwichChange;

use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;


#[derive(Debug)]
pub enum RepoFindError {
    Unknown(String),
    NotFound,
}

#[derive(Debug)]
pub enum RepoAddError {
    Unknown(String),
}

#[async_trait]
pub trait SandwichRepository {
    async fn find_sandwich(&self, sandwich_d: SandwichId) -> Result<Sandwich, RepoFindError>;

    async fn add_sandwich(&self, add_sandwich_change: AddSandwichChange) -> Result<(), RepoAddError>;
}