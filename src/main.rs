use std::env;

mod day1;

fn main() {
    let args: Vec<_> = env::args().collect();

    let problem_name = args[1].as_str();
    
    match problem_name {
        "1a" => day1::solve_a(),
        "1b" => day1::solve_b(),
        _ => ()
    }
}
