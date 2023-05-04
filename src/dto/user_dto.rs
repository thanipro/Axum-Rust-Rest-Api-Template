use crate::entity::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Password must be between 3 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserRegisterDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Password must be between 3 and 20 characters"
    ))]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub user_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReadDto {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: i8,
}

impl UserReadDto {
    pub fn from(model: User) -> UserReadDto {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            user_name: model.user_name,
            email: model.email,
            created_at: model.created_at,
            updated_at: model.updated_at,
            is_active: model.is_active,
        }
    }
}

impl std::fmt::Debug for UserLoginDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("email", &self.email)
            .finish()
    }
}

impl std::fmt::Debug for UserRegisterDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("user_name", &self.user_name)
            .field("email", &self.email)
            .finish()
    }
}