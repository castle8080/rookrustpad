pub mod test_handlers;
pub mod static_handler;

use actix_web::web;

use test_handlers::create_test_handlers;
use static_handler::create_static_handler;

pub fn configure_all_handlers(cfg: &mut web::ServiceConfig) {
    cfg
        .configure(create_test_handlers)
        .configure(create_static_handler);
}