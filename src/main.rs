use std::sync::Arc;
use crate::config::{database, parameter};
use crate::config::database::DatabaseTrait;

mod config;
mod routes;
mod dto;
mod error;
mod response;
mod entity;
mod repository;
mod state;
mod service;
mod middleware;

#[tokio::main]
async fn main() {
    parameter::init();
    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let host = format!("0.0.0.0:{}", parameter::get("PORT"));
    axum::Server::bind(&host.parse().unwrap())
        .serve(routes::root::routes(Arc::new(connection)))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
