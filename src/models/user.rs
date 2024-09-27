use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "Admin")]
    pub login: String,
    #[schema(example = "0")]
    pub role: String,
    #[schema(example = "admin@gmail.com")]
    pub email: Option<String>,
    #[schema(example = false )]
    pub status: bool,
}