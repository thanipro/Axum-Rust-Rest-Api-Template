use axum::{http::Request, middleware::Next, response::IntoResponse};

pub async fn logger<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    println!("{} {}", req.method(), req.uri());
    next.run(req).await
}
