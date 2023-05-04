use crate::error::{api_error::ApiError, token_error::TokenError, user_error::UserError};
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::token_state::TokenState;
use axum::extract::State;
use axum::{http, http::Request, middleware::Next, response::IntoResponse};
use jsonwebtoken::errors::ErrorKind;
use axum::headers::authorization::{Authorization, Bearer};
use axum::headers::Header;

pub async fn auth<B>(
    State(state): State<TokenState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header: Authorization<Bearer> = Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;
    let token = header.token();
     match state.token_service.retrieve_token_claims(token) {
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
                    _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
                };
            }
        }
}
