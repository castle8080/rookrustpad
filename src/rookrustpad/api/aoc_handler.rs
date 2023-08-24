use std::sync::Arc;

use actix_web::{get, web, Responder};

use crate::rookrustpad::service::aoc::AocService;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
struct AocAnswerRequest {
    year: u32,
    day: u32,
    part: u8,
    dataset: Option<String>,
}

#[get("problems")]
async fn get_problems(aoc_service: web::Data<Arc<AocService>>) -> impl Responder {
    web::Json(aoc_service.list_problems())
}

#[get("inputs")]
async fn get_inputs(aoc_service: web::Data<Arc<AocService>>) -> impl Responder {
    aoc_service.list_inputs().map(|inputs| web::Json(inputs))
}

#[get("answer/{year}/{day}/{part}")]
async fn get_answer(
    aoc_service: web::Data<Arc<AocService>>,
    request: web::Path<AocAnswerRequest>) -> impl Responder
{
    let answer = aoc_service.get_answer(request.year, request.day, request.part, &request.dataset);
    web::Json(answer)
}

pub fn create_aoc_handlers(cfg: &mut web::ServiceConfig) {
    println!("Creating AOC handlers");
    cfg.service(
        web::scope("/api/aoc")
            .service(get_problems)
            .service(get_inputs)
            .service(get_answer)
    );
}
