use crate::dto::token_dto::TokenReadDto;
use crate::dto::user_dto::UserLoginDto;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::error::user_error::UserError;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::auth_state::AuthState;
use axum::extract::State;
use axum::Json;

pub async fn auth(
    State(state): State<AuthState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginDto>,
) -> Result<Json<TokenReadDto>, ApiError> {
    let user = state
        .user_repo
        .find_by_email(payload.email)
        .await
        .ok_or(UserError::UserNotFound)?;

    return match state.user_service.verify_password(&user, &payload.password) {
        true => Ok(Json(state.token_service.generate_token(user)?)),
        false => Err(UserError::InvalidPassword)?,
    };
}
