use crate::dto::user_dto::UserReadDto;
use crate::entity::user::User;
use crate::response::api_response::ApiSuccessResponse;
use axum::{Extension, Json};

pub async fn profile(
    Extension(current_user): Extension<User>,
) -> Json<ApiSuccessResponse<UserReadDto>> {
    Json(ApiSuccessResponse::send(UserReadDto::from(current_user)))
}
