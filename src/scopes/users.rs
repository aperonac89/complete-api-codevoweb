use crate::{
    db::UserExt,
    dtos::{FilterUserDto, RequestQueryDto, UserData, UserListResponseDto, UserResponseDto},
    errors::HttpError,
    extractors::auth::{RequireAuth, RequireOnlyAdmin},
    models::User,
    AppState,
};
use actix_web::{web, HttpMessage, HttpRequest, Scope};
use validator::Validate;

pub fn users_scope() -> Scope {
    web::Scope("/api/users")
        .service("", web::get().to(get_users).wrap(RequireOnlyAdmin))
        .service("/me", web::get().to(get_me).wrap(RequireAuth))
}

pub async fn get_me(req: HttpRequest) -> Result<HttpResponse, HttpError> {
    match req.extensions_mut().get::<User>() {
        Some(user) => {
            let filtered_user = FilterUserDto::filter_user(user);

            let response_data = UserResponseDto {
                status: "Success".to_string(),
                data: UserData {
                    user: filtered_user,
                },
            };

            Ok(HttpResponse::Ok().json(response_data))
        }
        None => Err(HttpError::server_error("User not found")),
    }
}

pub async fn get_users(
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(UserListResponseDto {
        status: "Success".to_string(),
        users: FilterUserDto::filter_users(&users),
        result: users.len(),
    }))
}
