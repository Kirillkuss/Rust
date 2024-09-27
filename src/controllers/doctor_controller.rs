use actix_web::{get, web, HttpResponse, Responder};
use crate::database::AppState;
#[path = "../models/doctor.rs"] mod doctor;
#[path = "../response/error_response.rs"] mod error_response;

const CONTENT_TYPE: &str = "application/json; charset=UTF-8";

#[utoipa::path(
    get,
    path = "/doctors",
    responses(
        (status = 200, description = "Список врачей",  body = Doctor),
        (status = 400, description = "Плохой запрос",  body = ErrorResponse),
        (status = 500, description = "Ошибка сервера", body = ErrorResponse)
    )
)]
#[get("")]
async fn get_doctors(data: web::Data<AppState>) -> impl Responder {
    let doctors = sqlx::query_as::<_, doctor::Doctor>("SELECT * FROM doctor limit 20").fetch_all(&*data.db_pool).await;
    match doctors {
        Ok(doctors) => HttpResponse::Ok().content_type( CONTENT_TYPE ).json(doctors),
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            let error_response = error_response::ErrorResponse {
                error: "Ошибка доступа к базе данных".to_string(),
            };
            HttpResponse::NotFound().content_type( CONTENT_TYPE ).json(error_response)
        }
    }
}

pub fn get_api_routes_doctor() -> actix_web::Scope {
    web::scope("/doctors") //базовый путь
        .service(get_doctors )
}