use std::{future::Future, io, net::TcpListener, pin::Pin};

use sqlx::PgConnection;

pub type ServerFuture = Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'static>>;

#[derive(serde::Deserialize)]
struct SubscriptionRequest {
    email: String,
    name: String,
}

#[cfg(feature = "actix")]
mod actix_routes;

#[cfg(feature = "axum")]
mod axum_routes;

pub fn run(listener: TcpListener, connection: PgConnection) -> io::Result<ServerFuture> {
    #[cfg(feature = "actix")]
    {
        actix_routes::run(listener, connection)
    }
    #[cfg(feature = "axum")]
    {
        axum_routes::run(listener)
    }
    #[cfg(not(any(feature = "actix", feature = "axum")))]
    {
        compile_error!("Enable either the `actix` or `axum` feature.");
    }
}
