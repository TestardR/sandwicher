use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};

use crate::internal::application::query::get_sandwich::GetSandwich;
use crate::internal::application::service::sandwich_service::SandwichHandler;
use crate::internal::domain::sandwich::Sandwich;
use crate::internal::application::service::sandwich_service::GetError;
use crate::internal::infrastructure::rest::errors::ApiError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSandwichResponse {
    pub id: i64,
    pub name: String,
}

impl From<Sandwich> for GetSandwichResponse {
    fn from(s: Sandwich) -> Self {
        GetSandwichResponse {
            id: *s.id().value(),
            name: s.name().value().to_string(),
        }
    }
}

pub async fn get_sandwich<T: SandwichHandler>(
    handler: Data<T>,
    path: Path<String>,
) -> Result<Json<GetSandwichResponse>, ApiError>
{
    let query = GetSandwich::new(path.into_inner().parse::<i64>().unwrap());
    let result = handler.handle_get_sandwich(query).await;

    match result {
        Ok(sandwich) => Ok(Json(GetSandwichResponse::from(sandwich))),
        Err(e) => match e {
            GetError::NotFound => Err(ApiError::BadRequest(String::from("No sandwich found with the specified criteria"))),
            GetError::Unknown(e) => Err(ApiError::InternalServerError(e)),
        }
    }
}
