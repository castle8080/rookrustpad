use std::sync::Arc;

use actix_web::{get, web, Result as AwResult};
use serde::{Deserialize, Serialize};
use maud::{html, Markup};

use crate::rookrustpad::service::aoc::AocService;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AocAnswerRequest {
    year: u32,
    day: u32,
    part: u8,
}

#[get("/aoc")]
pub async fn index(aoc_service: web::Data<Arc<AocService>>) -> AwResult<Markup> {
    let problems = aoc_service.list_problems();

    Ok(html! {
        html {
            head {
                title { "Advent of Code" }
            }
            body {
                h1 { "Advent of Code" }
                ul {
                    @for problem in &problems {
                        li {
                            a href=(format!("/aoc_problems/{}/Day {} - Advent of Code {}.html", problem.year, problem.day, problem.year)) {
                                (format!("{} - Day {} - Part {}", problem.year, problem.day, problem.part))
                            }
                            " / "
                            a href=(format!("/aoc/answer/{}/{}/{}", problem.year, problem.day, problem.part)) {
                                "[Answer]"
                            }
                            " / " 
                            a href=(format!("/api/aoc/answer/{}/{}/{}", problem.year, problem.day, problem.part)) {
                                "[Answer API]"
                            }
                        }
                    }
                }
            }
        }
    })
}

#[get("/aoc/answer/{year}/{day}/{part}")]
pub async fn answer(
    aoc_service: web::Data<Arc<AocService>>,
    request: web::Path<AocAnswerRequest>) -> AwResult<Markup>
{
    let aoc_result = aoc_service.get_answer(request.year, request.day, request.part);

    Ok(html! {
        html {
            head {
                title { "Advent of Code Answer" }
            }
            body {
                h1 { (format!("AOC {} - Day {} - Part {}", request.year, request.day, request.part)) }
                div {
                    div {
                        @match &aoc_result.result {
                            Err(error) => {
                                b { "Error:"  } (error)
                            },
                            Ok(result) => {
                                b { "Result:" } (result)
                            }
                        }
                    }
                    br { }
                    b { "Execution Time: " } (aoc_result.execution_time) " seconds."
                    br { }
                    b { "Execution Log" }
                    br { }
                    textarea rows="15" cols="150" {
                        (&aoc_result.log)
                    }
                }
            }
        }
    })
}
