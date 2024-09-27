#[path = "../models/doctor.rs"] mod doctor;
use sqlx::PgPool;
use sqlx::Error;

pub struct DoctorService;

impl DoctorService {
    pub async fn get_doctors(db_pool: &PgPool, page: u32, size: u32) -> Result<Vec<doctor::Doctor>, Error> {
        let offset = (page - 1) * size;
        let doctors = sqlx::query_as::<_, doctor::Doctor>("SELECT * FROM doctor LIMIT $1 OFFSET $2")
        .bind(size as i32)
        .bind(offset as i32)
        .fetch_all(db_pool)
        .await?;
        
        Ok(doctors)
    }
}