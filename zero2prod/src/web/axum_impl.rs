use super::*;
use axum::{Form, Router, http::StatusCode, routing::get, routing::post};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(x: Form<SubscriptionRequest>) -> StatusCode {
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let serve = axum::serve(listener, app);
    Ok(Box::pin(async move { serve.await }))
}
