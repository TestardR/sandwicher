use serde::{Deserialize, Serialize};
use actix_web::web::{Data, Json};
use validator::Validate;

use crate::internal::application::command::create_sandwich::CreateSandwich;
use crate::internal::application::service::sandwich_service::{CreateError, SandwichHandler};
use crate::internal::infrastructure::rest::errors::ApiError;
use crate::internal::infrastructure::rest::validate::validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSandwichRequest {
    #[validate(length(
    min = 3,
    message = "name must be at least 3 characters"
    ))]
    pub name: String,
}

pub async fn create_sandwich<T: SandwichHandler>(
    request: Json<CreateSandwichRequest>,
    handler: Data<T>,
) -> Result<Json<()>, ApiError>
{
    validate(&request)?;

    let sandwich_name = &request.name;
    let command = CreateSandwich::new(sandwich_name.to_string());
    let result = handler.handle_create_sandwich(command).await;

    match result {
        Ok(_) => Ok(Json(())),
        Err(e) => match e {
            CreateError::DomainViolation(e) => Err(ApiError::BadRequest(e)),
            CreateError::Unknown(e) => Err(ApiError::InternalServerError(String::from("an error occurred"))),
        }
    }
}

