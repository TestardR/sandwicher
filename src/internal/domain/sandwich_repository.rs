use async_trait::async_trait;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoGetError {
    NotFound,
    Unknown(String)
}

#[async_trait]
pub trait Repository {
    async fn create(&self, sandwich: Sandwich) -> Result<(), RepoCreateError>;
    async fn find_one(&self, sandwich_d: SandwichId) -> Result<Sandwich, RepoGetError>;
}