use actix_web::{web};
use actix_files as fs;

pub fn create_static_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        fs::Files::new("/", "www")
            .show_files_listing()
            .index_file("index.html")
    );
}