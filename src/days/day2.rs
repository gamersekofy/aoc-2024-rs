/* Notes:
    Given a file with reports:
    12 18 27 7 19

    We have to determine if a level (line in report) is safe.
    Safe level:
        1. All are increasing or decreasing
        2. Adjacent levels diff d: 1 <= d <= 3

    How many reports are safe?
*/
use std::{fs, path::Path};

pub fn run(input_file_path: &str) {
    let file_contents =
        fs::read_to_string(Path::new(input_file_path)).expect("Unable to read file");
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in file_contents.lines() {
        reports.push(Part1::parse_report(line))
    }

    let safe_count = reports
        .iter()
        .filter(|report| Part2::is_safe_with_dampener(&report))
        .count();
    println!("Total number of safe reports with dampener: {}", safe_count);
}

pub struct Part1;
pub struct Part2;

impl Part1 {
    fn parse_report(report: &str) -> Vec<i32> {
        report
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    fn is_safe(report: &Vec<i32>) -> bool {
        match report.len() {
            1 => true,
            _ => {
                let mut increasing = true;
                let mut decreasing = true;

                for window in report.windows(2) {
                    let diff = window[1] - window[0];
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                    }

                    if diff > 0 {
                        decreasing = false;
                    } else {
                        increasing = false;
                    }

                    if !increasing && !decreasing {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl Part2 {
    fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
        if Part1::is_safe(report) {
            return true;
        }

        for skip_index in 0..report.len() {
            let modified_report: Vec<i32> = report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_index)
                .map(|(_, &x)| x)
                .collect();

            if Part1::is_safe(&modified_report) {
                return true;
            }
        }
        false
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_parse_report_normal() {
        let raw_report = "7 6 4 2 1";
        let pretty_report = vec![7, 6, 4, 2, 1];

        assert_eq!(Part1::parse_report(raw_report), pretty_report);
    }

    #[test]
    fn test_parse_report_single() {
        let raw_report = "10";
        let pretty_report = vec![10];

        assert_eq!(Part1::parse_report(raw_report), pretty_report);
    }
}
