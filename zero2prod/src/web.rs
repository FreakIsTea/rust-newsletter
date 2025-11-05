// web/mod.rs
use std::{future::Future, io, net::TcpListener, pin::Pin};

pub type ServerFuture = Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'static>>;

pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
    #[cfg(feature = "actix")]
    {
        actix_impl::run(listener)
    }
    #[cfg(all(not(feature = "actix"), feature = "axum"))]
    {
        axum_impl::run(listener)
    }
    #[cfg(not(any(feature = "actix", feature = "axum")))]
    {
        compile_error!("Enable either the `actix` or `axum` feature.");
    }
}

#[cfg(feature = "actix")]
mod actix_impl {
    use super::*;
    use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

    async fn health_check() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
        let server: Server =
            HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
                .listen(listener)?
                .run();

        Ok(Box::pin(server))
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use super::*;
    use axum::{Router, http::StatusCode, routing::get};

    async fn health_check() -> StatusCode {
        StatusCode::OK
    }

    pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
        listener.set_nonblocking(true)?;
        let listener = tokio::net::TcpListener::from_std(listener)?;
        let app = Router::new().route("/health_check", get(health_check));
        let serve = axum::serve(listener, app);
        Ok(Box::pin(async move { serve.await }))
    }
}
