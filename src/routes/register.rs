use crate::handler::register_handler;
use crate::state::user_state::UserState;
use axum::{routing::post, Router};

pub fn routes() -> Router<UserState> {
    let router = Router::new().route("/register", post(register_handler::register));
    return router;
}
