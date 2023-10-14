use async_trait::async_trait;
use sqlx::{Error, Pool, Sqlite};
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;
use crate::internal::domain::sandwich_repository::{RepoCreateError, RepoGetError, Repository};

const SANDWICH_TABLE: &str = "sandwich";
const SANDWICH_ID_FIELD: &str = "id";
const SANDWICH_NAME_FIELD: &str = "name";
const SANDWICH_INGREDIENTS_FIELD: &str = "ingredients";
const SANDWICH_TYPE_FIELD: &str = "sandwich_type";

#[derive(Clone)]
pub struct SandwichStore {
    sql_lite: Pool<Sqlite>,
}

impl SandwichStore {
    pub fn new(sql_lite: Pool<Sqlite>) -> Self {
        Self {
            sql_lite
        }
    }
}

#[async_trait]
impl Repository for SandwichStore {
    async fn create(&self, sandwich: Sandwich) -> Result<(), RepoCreateError> {
        let query = format!("INSERT INTO {} ({}, {}, {}) VALUES (?, ?, ?)",
                            SANDWICH_TABLE, SANDWICH_NAME_FIELD, SANDWICH_INGREDIENTS_FIELD, SANDWICH_TYPE_FIELD);

        let result = sqlx::query(&query)
            .bind(sandwich.name().value())
            .bind(sandwich.ingredients())
            .bind(sandwich.sandwich_type())
            .execute(&self.sql_lite)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoCreateError::Unknown(e.to_string())),
        }
    }

    async fn find_one(&self, sandwich_id: SandwichId) -> Result<Sandwich, RepoGetError> {
        let id = sandwich_id.value().unwrap();

        let query = format!("SELECT {}, {}, {}, {} FROM {} WHERE id = {}",
                            SANDWICH_ID_FIELD, SANDWICH_NAME_FIELD, SANDWICH_INGREDIENTS_FIELD, SANDWICH_TYPE_FIELD, SANDWICH_TABLE, id);

        let result = sqlx::query(&query)
            .fetch(&self.sql_lite)
            .await;

        result
            .map(|entity| {
                Ok(entity.try_into().unwrap())
            })
            .map_err(|e| match e {
                Error::RowNotFound => RepoGetError::NotFound,
                e => RepoGetError::Unknown(e.to_string()),
            })?
    }
}
