use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Doctor {
    #[schema(example = 1)]
    pub id_doctor: i32,

    #[schema(example = "Петров")]
    pub surname: String,

    #[schema(example = "Петр")]
    pub name: String,

    #[schema(example = "Петрович")]
    pub full_name: String,
}