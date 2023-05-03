use crate::config::database::Database;
use crate::repository::user_repository;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::{TokenService, TokenServiceTrait};
use crate::service::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub(crate) token_service: TokenService,
    pub(crate) user_repo: user_repository::UserRepository,
    pub(crate) user_service: UserService,
}

impl AuthState {
    pub fn new(db_conn: &Arc<Database>) -> AuthState {
        Self {
            token_service: TokenService::new(),
            user_service: UserService::new(db_conn),
            user_repo: user_repository::UserRepository::new(db_conn),
        }
    }
}
