use std::fs;

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn main() -> anyhow::Result<()> {
    let input =
        fs::read_to_string("../fixtures/day3").context("Should have been able to read the file")?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();

    let operations = lines
        .iter()
        .map(|line| process_line(line))
        .collect::<Vec<_>>();

    operations
        .iter()
        .flatten()
        .map(|op| match op {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn process_line(line: &str) -> Vec<Instruction> {
    let (_, operations) = many1(many_till(anychar, mul).map(|(_, op)| op))(line).unwrap();
    operations
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    // get all mul items
    let (input, _) = tag("mul")(input)?;
    // get all the pairs in the mul item
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::from(pair)))
}

fn mul_instruction(input: &str) -> IResult<&str, (u32, u32)> {
    // get all mul items
    let (input, _) = tag("mul")(input)?;
    // get all the pairs in the mul item
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, pair))
}

#[derive(Debug, Clone)]
enum Instruction {
    Dont,
    Do,
    Mul(u32, u32),
}

impl From<(u32, u32)> for Instruction {
    fn from(pair: (u32, u32)) -> Self {
        Instruction::Mul(pair.0, pair.1)
    }
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn process_line_2(line: &str) -> Vec<Instruction> {
    let (_, operations) = many1(many_till(anychar, instruction).map(|(_, op)| op))(line).unwrap();
    operations
}

fn part_two(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();

    let operations = lines.into_iter().map(process_line_2).collect::<Vec<_>>();

    let mut currentStatus = Instruction::Do;

    let mut sum = 0;
    for op in operations.into_iter().flatten() {
        match op {
            Instruction::Do => currentStatus = Instruction::Do,
            Instruction::Dont => currentStatus = Instruction::Dont,
            Instruction::Mul(a, b) => {
                if let Instruction::Do = currentStatus {
                    sum += a * b;
                }
            }
        }
    }

    sum
}

#[derive(Debug)]
enum Operation {
    Mul(u32, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part_one(input));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part_two(input));
        Ok(())
    }
}
