use actix_web::{get, web, HttpResponse, Responder};
use crate::database::AppState;
#[path = "../models/doctor.rs"] mod doctor;
#[path = "../response/error_response.rs"] mod error_response;
#[path = "../service/doctor_service.rs"] mod doctor_service;

const CONTENT_TYPE: &str = "application/json; charset=UTF-8";

#[utoipa::path(
    get,
    path = "/doctors/{page}/{size}",
    responses(
        (status = 200, description = "Список врачей",  body = Doctor),
        (status = 400, description = "Плохой запрос",  body = ErrorResponse),
        (status = 500, description = "Ошибка сервера", body = ErrorResponse)
    ),
    params(
        ("page" = u32, Path, description = "Страница"), 
        ("size" = u32, Path, description = "Размер")    
    )
    
)]
#[get("/{page}/{size}")]
async fn get_doctors(data: web::Data<AppState>, path: web::Path<(u32, u32)>) -> impl Responder {
    let (page, size) = path.into_inner();
    match doctor_service::DoctorService::get_doctors(&*data.db_pool, page, size).await {
        Ok(doctors) => HttpResponse::Ok().content_type(CONTENT_TYPE).json(doctors),
        Err(e) => {
            println!("Error: {:?}", e);
            let error_response = error_response::ErrorResponse {
                error: "Ошибка доступа к базе данных".to_string(),
            };
            HttpResponse::InternalServerError().content_type(CONTENT_TYPE).json(error_response)
        }
    }
}



pub fn get_api_routes_doctor() -> actix_web::Scope {
    web::scope("/doctors") //базовый путь
        .service(get_doctors )
}