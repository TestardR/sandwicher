use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqliteQueryResult;

use crate::internal::domain::add_sandwich_change::AddSandwichChange;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::domain::sandwich_id::SandwichId;
use crate::internal::domain::sandwich_repository::{RepoAddError, RepoFindError, SandwichRepository};
use crate::internal::infrastructure::store::sandwich::SandwichEntity;

const SANDWICH_TABLE: &str = "sandwich";
const SANDWICH_ID_FIELD: &str = "id";
const SANDWICH_NAME_FIELD: &str = "name";

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
impl SandwichRepository for SandwichStore {
    async fn find_sandwich(&self, sandwich_id: SandwichId) -> Result<Sandwich, RepoFindError> {
        let query = format!("SELECT {}, {} FROM {} WHERE id = {}",
                            SANDWICH_ID_FIELD, SANDWICH_NAME_FIELD, SANDWICH_TABLE, sandwich_id.value());

        let result: Result<SandwichEntity, sqlx::Error> = sqlx::query_as(&query)
            .fetch_one(&self.sql_lite).await;

        match result {
            Ok(entity) => Ok(entity.from_entity_to_domain_model()),
            Err(sqlx::Error::RowNotFound) => Err(RepoFindError::NotFound),
            Err(e) => Err(RepoFindError::Unknown(e.to_string()))
        }
    }

    async fn add_sandwich(&self, change: AddSandwichChange) -> Result<(), RepoAddError> {
        let query = format!("INSERT INTO {} ({}) VALUES (?)",
                            SANDWICH_TABLE, SANDWICH_NAME_FIELD);

        let result: Result<SqliteQueryResult, sqlx::Error>= sqlx::query(&query)
            .bind(change.name().value())
            .execute(&self.sql_lite)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoAddError::Unknown(e.to_string()))
        }
    }
}
