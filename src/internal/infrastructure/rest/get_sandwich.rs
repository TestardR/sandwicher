use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};

use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::application::service::sandwich_service::SandwichHandler;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::application::service::sandwich_service::GetError;
use crate::internal::domain::sandwich_type::SandwichType;
use crate::internal::infrastructure::rest::errors::ApiError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSandwichResponse {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}

impl From<Sandwich> for GetSandwichResponse {
    fn from(s: Sandwich) -> Self {
        GetSandwichResponse {
            id: s.id().value().clone().unwrap_or(String::from("")).to_string(),
            name: s.name().value().to_string(),
            ingredients: s.ingredients().value().clone(),
            sandwich_type: s.sandwich_type().clone(),
        }
    }
}

pub async fn get_sandwich<T: SandwichHandler>(
    handler: Data<T>,
    path: Path<String>,
) -> Result<Json<GetSandwichResponse>, ApiError>
{
    let sandwich_id = path.into_inner();
    let query = GetSandwich::new(&sandwich_id);
    let result = handler.handle_get_sandwich(query);

    result
        .map(|v| Ok(Json(GetSandwichResponse::from(v))))
        .map_err(|e| match e {
            GetError::Unknown(m) => ApiError::InternalServerError(m),
            GetError::NotFound => ApiError::BadRequest(String::from("No sandwich found with the specified criteria")),
        })?
}