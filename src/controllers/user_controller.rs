use actix_web::{get, web, HttpResponse, Responder};
use crate::database:: AppState ;
#[path = "../models/user.rs"] mod user;
#[path = "../response/error_response.rs"] mod error_response;

const CONTENT_TYPE: &str = "application/json; charset=UTF-8";

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Список пользователей", body = User ),
        (status = 400, description = "Плохой запрос",        body = ErrorResponse ),
        (status = 500, description = "Ошибка сервера",       body = ErrorResponse )
    )
)]
#[get("")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = sqlx::query_as::<_, user::User>("SELECT * FROM kl_user")
        .fetch_all(&*data.db_pool)
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().content_type( CONTENT_TYPE ).json(users),
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            let error_response = error_response::ErrorResponse {
                error: "Ошибка доступа к базе данных".to_string(),
            };
            HttpResponse::NotFound().content_type( CONTENT_TYPE ).json(error_response)
        }
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Поиск польз по ИД", body = User ),
        (status = 400, description = "Плохой запрос",     body = ErrorResponse ),
        (status = 500, description = "Ошибка сервера",    body = ErrorResponse )
    ),
    params(
        ("id" = u64, Path, description = "ИД пользователя"),
    )
)]
#[get("/{id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let user = sqlx::query_as::<_, user::User>("SELECT * FROM kl_user WHERE id = $1")
        .bind( path.into_inner())
        .fetch_one(&*data.db_pool)
        .await;
    match user {
        Ok(user) => HttpResponse::Ok().content_type( CONTENT_TYPE ).json(user),
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            let error_response = error_response::ErrorResponse {
                error: "Пользователь не найден".to_string(),
            };
            HttpResponse::NotFound().content_type( CONTENT_TYPE ).json(error_response)
        }
    }
}


pub fn get_api_routes() -> actix_web::Scope {
    web::scope("/users") //базовый путь
        .service(get_users )
        .service(get_user )
}