use std::{env, net::TcpListener};
use zero2prod::{configuration::get_configuration, web};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind address.");
    let addr = listener.local_addr()?;
    let server = web::run(listener)?;

    println!("Listenting on http://{}", addr);
    server.await
}
