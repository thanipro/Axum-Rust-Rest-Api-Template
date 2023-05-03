use std::sync::Arc;
use crate::config::database::Database;
use axum::routing::{get, IntoMakeService};
use axum::{middleware, Router};

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {

    let app_router = Router::new()
        .nest("/api", Router::new().route("/health", get(|| async { "Healthy..." })));

    app_router.into_make_service()
}
