use std::{future::Future, io, net::TcpListener, pin::Pin};

pub type ServerFuture = Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'static>>;

#[derive(serde::Deserialize)]
struct SubscriptionRequest {
    email: String,
    name: String,
}

#[cfg(feature = "actix")]
mod actix_impl;

#[cfg(feature = "axum")]
mod axum_impl;

pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
    #[cfg(feature = "actix")]
    {
        actix_impl::run(listener)
    }
    #[cfg(feature = "axum")]
    {
        axum_impl::run(listener)
    }
    #[cfg(not(any(feature = "actix", feature = "axum")))]
    {
        compile_error!("Enable either the `actix` or `axum` feature.");
    }
}
