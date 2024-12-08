use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<_> = env::args().collect();

    let problem_name = args[1].as_str();

    match problem_name {
        "1a" => day1::solve_a(),
        "1b" => day1::solve_b(),
        "2a" => day2::solve_a(),
        "2b" => day2::solve_b(),
        "3a" => day3::solve_a(),
        "3b" => day3::solve_b(),
        "4a" => day4::solve_a(),
        "4b" => day4::solve_b(),
        _ => (),
    }
}
