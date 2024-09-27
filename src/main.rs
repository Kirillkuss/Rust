use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[path = "controllers/doctor_controller.rs"] mod doctor_controller;
#[path = "controllers/user_controller.rs"] mod user_controller;
#[path = "models/doctor.rs"] mod doctor;
#[path = "models/user.rs"] mod user;
#[path = "response/error_response.rs"] mod error_response;
#[path = "config/database.rs"] mod database;


#[derive(OpenApi)]
#[openapi(
    info(description = "My Rust Api Klinika", title = "КЛИНИК АПИ", version = "1.0.0"),
    paths(user_controller::get_user, user_controller::get_users, doctor_controller::get_doctors),
    components(schemas( doctor::Doctor, user::User, error_response::ErrorResponse))
)]
pub struct ApiDoc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // загружаем переменные окружения
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = database::create_db_pool(&database_url).await;
    let data = web::Data::new(database::AppState { db_pool, });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(user_controller::get_api_routes())
            .service(doctor_controller::get_api_routes_doctor())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))

    })
    .bind("127.0.0.1:8070")?
    .run()
    .await
}