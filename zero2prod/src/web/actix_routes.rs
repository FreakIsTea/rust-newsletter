use super::*;
use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

pub fn run(listener: TcpListener, connection: PgConnection) -> io::Result<ServerFuture> {
    let connection = web::Data::new(connection);
    let server: Server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(Box::pin(server))
}
