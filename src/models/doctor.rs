use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Doctor {
    
    #[schema(value_type = i32, example = 1)]
    #[sqlx(rename = "id_doctor")]
    pub id: i32,

    #[schema( value_type = String, example = "Петров" )]
    pub surname: String,

    #[schema( value_type = String, example = "Петр" )]
    pub name: String,

    #[schema( value_type = String, example = "Петрович" )]
    #[sqlx(rename = "full_name")]
    pub fullname: String,
}