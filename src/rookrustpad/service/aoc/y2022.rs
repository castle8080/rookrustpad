use crate::rookrustpad::service::aoc::AocService;

pub mod day11;

pub fn configure_service(aoc_service: &mut AocService) {
    aoc_service.register_answer(2022, 11, 1, day11::part1);
    aoc_service.register_answer(2022, 11, 2, day11::part2);
}
