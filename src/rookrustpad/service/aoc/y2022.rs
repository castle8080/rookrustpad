use crate::rookrustpad::service::aoc::{AocFunction, AocProblem, AocResult, AocService};

pub mod day11;

use day11::part1;

pub fn configure_service(aoc_service: &mut AocService) {
    aoc_service.register_answer(2022, 11, 1, part1)
}
