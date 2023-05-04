use crate::error::{api_error::ApiError, token_error::TokenError, user_error::UserError};
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::token_state::TokenState;
use axum::extract::State;
use axum::{http, http::Request, middleware::Next, response::IntoResponse};
use jsonwebtoken::errors::ErrorKind;

pub async fn auth<B>(
    State(state): State<TokenState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|header_value| {
            let bearer = "Bearer ";
            if header_value.starts_with(bearer) {
                header_value[bearer.len()..].to_string()
            } else {
                header_value.to_string()
            }
        });

    return match token {
        Some(token) => match state.token_service.retrieve_token_claims(&token) {
            Ok(token_data) => {
                let user = state.user_repo.find_by_email(token_data.claims.email).await;
                match user {
                    Some(user) => {
                        req.extensions_mut().insert(user);
                        Ok(next.run(req).await)
                    }
                    None => return Err(UserError::UserNotFound)?,
                }
            }
            Err(err) => {
                return match err.kind() {
                    ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                    _ => Err(TokenError::InvalidToken(token))?,
                };
            }
        },
        _ => return Err(TokenError::MissingToken)?,
    };
}
