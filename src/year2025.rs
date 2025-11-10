use core::panic;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(day: u8, part: Option<u8>, input: &str) {
    match day {
        1 => day1::run(part, input),
        2 => day2::run(part, input),
        3 => day3::run(part, input),
        4 => day4::run(part, input),
        5 => day5::run(part, input),
        6 => day6::run(part, input),
        7 => day7::run(part, input),
        8 => day8::run(part, input),
        9 => day9::run(part, input),
        10 => day10::run(part, input),
        11 => day11::run(part, input),
        12 => day12::run(part, input),
        _ => panic!("tried to run a day above 12!"),
    }
}
