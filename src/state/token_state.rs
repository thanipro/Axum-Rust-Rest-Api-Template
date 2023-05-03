use crate::config::database::Database;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::service::token_service::{TokenService, TokenServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct TokenState {
    pub token_service: TokenService,
    pub user_repo: UserRepository,
}

impl TokenState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            token_service: TokenService::new(),
            user_repo: UserRepository::new(db_conn),
        }
    }
}
