use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

pub async fn create_db_pool(database_url: &str) -> Arc<PgPool> {
    let db_pool = PgPool::connect(database_url)
        .await
        .expect("Failed to create pool.");
    Arc::new(db_pool)
}
