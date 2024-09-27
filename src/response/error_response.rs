use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Not found!")]
    pub error: String,
}