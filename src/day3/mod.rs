use std::fs;

use regex::Regex;

pub fn solve_a() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let concat = contents.lines().fold(String::new(), |acc, line| format!("{acc}{line}"));
    let results = parse_mul(&concat.as_str());

    println!("{results}");
}

pub fn solve_b() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let concat = contents.lines().fold(String::new(), |acc, line| format!("{acc}{line}"));

    let mut results = 0;
    // [[yes...], [no, yes..], [no, yes...]]
    let split: Vec<_> = concat.split("don't()").map(|substring| substring.split("do()").collect::<Vec<_>>()).collect();
    for (index, group) in split.iter().enumerate() {
        if index == 0 {
            results += group.iter().map(|&content| parse_mul(content)).sum::<i32>();
            continue;
        }
        results += group.iter().skip(1).map(|&content| parse_mul(content)).sum::<i32>();
    }

    println!("{results}");
}

pub fn parse_mul(content: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = 0;
    for (_, [number1, number2]) in re.captures_iter(content).map(|capture| capture.extract()) {
        results += number1.parse::<i32>().unwrap() * number2.parse::<i32>().unwrap();
    }
    results
}