use std::fs;

pub fn solve_a() {
    let reports = read_reports();
    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe(&report))
        .count();

    println!("{:?}", safe_reports);
}

pub fn solve_b() {
    let reports = read_reports();
    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe_with_dampener(&report))
        .count();

    println!("{:?}", safe_reports);
}

fn read_reports() -> Vec<Vec<i32>> {
    let contents = fs::read_to_string("inputs/day2.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let entry: Vec<_> = line
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect();
            entry
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut diff: i32;
    let mut was_positive: Option<bool> = Option::None;

    for (index, &level) in report.iter().enumerate() {
        if index == report.len() - 1 {
            return true;
        }
        diff = report[index + 1] - level;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match was_positive {
            Option::None => {
                was_positive = Option::Some(diff.is_positive());
            }
            Option::Some(true) => {
                if diff.is_negative() {
                    return false;
                }
            }
            Option::Some(false) => {
                if diff.is_positive() {
                    return false;
                }
            }
        }
    }
    true
}

fn is_report_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true;
    }

    let alternative_reports: Vec<Vec<i32>> = (0..report.len())
        .map(|index| {
            report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, l)| *l)
                .collect()
        })
        .collect();

    for alt_report in alternative_reports.iter() {
        if is_report_safe(alt_report) {
            return true;
        }
    }
    return false;
}
