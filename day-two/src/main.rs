use std::fs;

use anyhow::Context;
use itertools::Itertools;

pub fn main() -> anyhow::Result<()> {
    let input =
        fs::read_to_string("../fixtures/day2").context("Should have been able to read the file")?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let reports: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports.into_iter().filter(|r| is_report_valid(r)).count()
}

fn part_two(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();

    let reports: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports
        .into_iter()
        .filter(|r| is_report_valid_dampner(r.to_vec()))
        .count()
}

fn is_difference_valid(a: i32, b: i32) -> bool {
    let difference = (a - b).abs();

    if !(1..=3).contains(&difference) {
        return false;
    }

    true
}

fn is_report_valid(report: &[i32]) -> bool {
    let mut is_increasing = false;
    let mut is_decreasing = false;
    for i in 0..report.len() - 1 {
        if report[i] > report[i + 1] {
            if is_decreasing {
                return false;
            }

            is_increasing = true;
            if !is_difference_valid(report[i], report[i + 1]) {
                return false;
            }
            continue;
        }

        if report[i] < report[i + 1] {
            if is_increasing {
                return false;
            }
            is_decreasing = true;
            if !is_difference_valid(report[i], report[i + 1]) {
                return false;
            }
            continue;
        }

        return false;
    }

    true
}

fn is_report_valid_dampner(report: Vec<i32>) -> bool {
    let report_len = report.len();
    // Some combination of the vec missing up to 1 element must be valid.
    report
        .into_iter()
        .combinations(report_len - 1)
        .any(|c| is_report_valid(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_report_valid_test() {
        let report = [1, 2, 3, 4, 5];
        assert!(is_report_valid(&report));

        let report = [1, 2, 1, 2, 3];
        assert!(!is_report_valid(&report));
    }

    #[test]
    fn test_part_one() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_two(input), 4);
    }

    #[test]
    fn test_report_valid_dampner() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_report_valid_dampner(report));
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_report_valid_dampner(report));
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_report_valid_dampner(report));
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_report_valid_dampner(report));
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_report_valid_dampner(report));
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_report_valid_dampner(report));
    }
}
