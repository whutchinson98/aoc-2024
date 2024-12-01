use std::{collections::HashMap, fs};

use anyhow::Context;

pub fn main() -> anyhow::Result<()> {
    let input =
        fs::read_to_string("../fixtures/day1").context("Should have been able to read the file")?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

pub fn part_one(input: &str) -> i64 {
    let mut lines = input.lines().collect::<Vec<_>>();

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in lines.iter_mut() {
        let parts = line.split_once("   ").unwrap();

        a.push(parts.0.parse::<i64>().unwrap());
        b.push(parts.1.parse::<i64>().unwrap());
    }

    a.sort();
    b.sort();

    let diffs = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .collect::<Vec<_>>();

    let sum = diffs.iter().sum::<i64>();

    sum
}

pub fn part_two(input: &str) -> i64 {
    let mut lines = input.lines().collect::<Vec<_>>();
    let mut seen: HashMap<i64, i64> = HashMap::new();

    let mut b = Vec::new();

    for line in lines.iter_mut() {
        let parts = line.split_once("   ").unwrap();

        seen.insert(parts.0.parse::<i64>().unwrap(), 0);
        b.push(parts.1.parse::<i64>().unwrap());
    }

    b.into_iter().for_each(|b| {
        if seen.contains_key(&b) {
            *seen.get_mut(&b).unwrap() += 1;
        }
    });

    seen.into_iter().map(|(k, v)| k * v).sum::<i64>()
}
