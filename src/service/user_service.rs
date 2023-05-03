use crate::config::database::{Database, DatabaseTrait};
use crate::dto::user_dto::{UserReadDto, UserRegisterDto};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use sqlx::Error as SqlxError;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    db_conn: Arc<Database>,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_user(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        return match self.user_repo.find_by_email(payload.email.to_owned()).await {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                let user = self.add_user(payload).await;

                return match user {
                    Ok(user) => Ok(UserReadDto::from(user)),
                    Err(e) => match e {
                        SqlxError::Database(e) => match e.code() {
                            Some(code) => {
                                if code == "23000" {
                                    Err(DbError::UniqueConstraintViolation(e.to_string()))?
                                } else {
                                    Err(DbError::SomethingWentWrong(e.to_string()))?
                                }
                            }
                            _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                        },
                        _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                    },
                };
            }
        };
    }

    async fn add_user(&self, payload: UserRegisterDto) -> Result<User, SqlxError> {
        let insert = sqlx::query_as!(
            User,
            r#"
        INSERT INTO user (first_name, last_name, user_name, email, password, is_active)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
            payload.first_name,
            payload.last_name,
            payload.user_name,
            payload.email,
            bcrypt::hash(payload.password, 4).unwrap(),
            1
        )
        .execute(self.db_conn.get_pool())
        .await?;

        let user = self.user_repo.find(insert.last_insert_id()).await?;
        return Ok(user);
    }

    pub fn verify_password(&self, user: &User, password: &str) -> bool {
        return bcrypt::verify(password, &user.password).unwrap_or(false);
    }
}
