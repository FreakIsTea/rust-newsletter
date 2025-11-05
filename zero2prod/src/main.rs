use std::{env, net::TcpListener};

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let port: i32 = args.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind address.");
    run(listener)?.await
}
