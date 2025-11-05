use super::*;
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

pub fn run(listener: TcpListener) -> io::Result<ServerFuture> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(Box::pin(server))
}
