use std::time::SystemTimeError;

use axum::response::IntoResponse;
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("SystemTime error: {0}")]
    DataTime(#[from] SystemTimeError),
    #[error("Token expired")]
    Expired,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Refresh token not found in database")]
    RefreshNotFound,
}

