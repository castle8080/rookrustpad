mod rookrustpad;

use actix_web::{App, HttpServer, middleware};

use rookrustpad::api::configure_all_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = "127.0.0.1";
    let listen_port = 8080;

    env_logger::init();

    println!("Starting server on {}:{}", listen_addr, listen_port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(configure_all_handlers)
    })
    .bind((listen_addr, listen_port))?
    .run()
    .await
}
