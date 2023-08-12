mod rookrustpad;

use actix_web::{App, HttpServer, middleware};
use rookrustpad::app_configuration::AppConfiguration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = "127.0.0.1";
    let listen_port = 8080;

    env_logger::init();

    println!("Starting server on {}:{}", listen_addr, listen_port);

    let app_config = AppConfiguration::create_default();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(|cfg| app_config.configure(cfg))
    })
    .bind((listen_addr, listen_port))?
    .run()
    .await
}
