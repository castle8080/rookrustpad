use actix_web::web;
use std::sync::Arc;

use crate::rookrustpad::page::configure_page_handlers;

use crate::rookrustpad::service::aoc::AocService;

use crate::rookrustpad::api::aoc_handler::create_aoc_handlers;
use crate::rookrustpad::api::static_handler::create_static_handler;
use crate::rookrustpad::api::test_handlers::create_test_handlers;

#[derive(Clone)]
pub struct AppConfiguration {
    aoc_service: Arc<AocService>,
}

impl AppConfiguration {
    pub fn create_default() -> AppConfiguration {
        AppConfiguration {
            aoc_service: Arc::new(AocService::create_default(String::from("www/aoc_input"))),
        }
    }

    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        println!("Configuring app data");
        cfg.app_data(web::Data::new(self.aoc_service.clone()));

        println!("Configuring all handlers");
        cfg.configure(configure_page_handlers);
        cfg.configure(create_test_handlers);
        cfg.configure(create_aoc_handlers);
        cfg.configure(create_static_handler);
    }
}
