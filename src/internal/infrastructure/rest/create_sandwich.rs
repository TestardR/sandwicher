use serde::{Deserialize, Serialize};

use crate::internal::domain::sandwich_type::SandwichType;
use actix_web::web::{Data, Json};

use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::service::sandwich_service::{SandwichHandler};
use crate::internal::domain::sandwich_repository::{CreateError};

use crate::internal::infrastructure::rest::errors::ApiError;
use crate::internal::infrastructure::rest::helpers::{string_vec_to_vec_str};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSandwichRequest {
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}

pub async fn create_sandwich<T: SandwichHandler>(
    request: Json<CreateSandwichRequest>,
    handler: Data<T>,
) -> Result<Json<()>, ApiError>
{
    let ingredients = string_vec_to_vec_str(&request.ingredients);
    let command = CreateSandwich::new(
        &request.name,
        &ingredients,
        &request.sandwich_type,
    );
    let result = handler.handle_create_sandwich(command);

    result
        .map(|_| {
            Ok(Json(()))
        })
        .map_err(|e| match e {
            CreateError::Unknown(e) => ApiError::InternalServerError(e),
            CreateError::InvalidData(e) => ApiError::BadRequest(e),
        })?
}

