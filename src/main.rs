use std::net::TcpListener;

use rs_zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener =
        TcpListener::bind("127.0.0.1:8000").expect("Failed to listen to standard port (8000).");

    run(listener)?.await
}