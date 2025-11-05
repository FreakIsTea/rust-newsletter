use super::*;
use axum::{Form, Router, http::StatusCode, routing::get, routing::post};

mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let serve = axum::serve(listener, app);
    Ok(Box::pin(async move { serve.await }))
}
