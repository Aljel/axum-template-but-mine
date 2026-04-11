use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use utoipa::OpenApi;

use crate::{
    AppState,
    errors::auth::AuthError,
    repository::{is_unique_violation, users::UserRepository},
    schemas::users::{LoginUser, RegisterUser},
    services::auth::hashing::hash,
};

pub struct AuthRouter;

impl AuthRouter {
    pub fn set_router() -> Router<AppState> {
        Router::new()
            .route("/registration", post(register))
            .route("/login", post(login))
    }
}

#[derive(OpenApi)]
#[openapi(paths(register, login), components(schemas(LoginUser, RegisterUser)))]
pub struct AuthDocs;

// TODO: вынести логику в services

#[utoipa::path(
    post,
    path = "/registration",
    request_body = RegisterUser,
    responses(
        (status = 200, description = "Пользователь зарегестрирован", body = (String, String)),
        (status = 409, description = "Пользователь с такой почтой уже существует", body = String),
        (status = 500, description = "Технические шокаладки с бд", body = String)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterUser>,
) -> Result<impl IntoResponse, AuthError> {
    let repo = state.user_repo.clone();
    match repo
        .create(repo.db_pool.clone().as_ref(), user_data.clone())
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json((user_data.email.clone(), user_data.email)),
        )),
        Err(e) if is_unique_violation(&e) => Err(AuthError::UserAlreadyExists),
        Err(e) => Err(AuthError::Db(e)),
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Вход успешен", body = (String, String)),
        (status = 404, description = "Пользователь не найден", body = String),
        (status = 500, description = "Технические шокаладки с бд", body = String)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(user_data): Json<LoginUser>,
) -> Result<impl IntoResponse, AuthError> {
    let repo = state.user_repo.clone();

    if repo
        .check_login(&user_data.email, &hash(&user_data.password))
        .await?
        .is_some()
    {
        return Ok((
            StatusCode::OK,
            Json((user_data.email.clone(), user_data.email)),
        ));
    }

    Err(AuthError::Unauthorized)
}
