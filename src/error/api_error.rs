use crate::error::{db_error::DbError,token_error::TokenError, user_error::UserError };
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DbError(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TokenError(error) => error.into_response(),
            ApiError::UserError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
        }
    }
}
