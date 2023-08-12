pub mod test_handlers;
pub mod static_handler;
pub mod aoc_handler;

use actix_web::web;

use test_handlers::create_test_handlers;
use static_handler::create_static_handler;
use aoc_handler::create_aoc_handlers;

pub fn configure_all_handlers(cfg: &mut web::ServiceConfig) {
    println!("Configuring all handlers");
    cfg
        .configure(create_test_handlers)
        .configure(create_aoc_handlers)
        .configure(create_static_handler);
}
