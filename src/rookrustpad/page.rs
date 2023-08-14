use actix_web::web;

mod index;
mod aoc;

pub fn configure_page_handlers(cfg: &mut web::ServiceConfig) {
    println!("Configuring page handlers");
    cfg
        .service(index::index)
        .service(aoc::index)
        .service(aoc::answer);
}
