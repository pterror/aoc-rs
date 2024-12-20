use std::fmt::Debug;

use anyhow::Result;
use regex::Regex;

use crate::util::*;

pub struct Day3;

impl Solution for Day3 {
    type Input = Vec<u8>;

    fn day() -> u8 {
        3
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day3.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.clone().ok()
    }

    fn p1(str: Self::Input) -> Result<impl Debug> {
        let pat = Regex::new(r"mul\((\d+),(\d+)\)")?;
        let mut total = 0isize;
        for instr in pat.captures_iter(&str.to_str()) {
            total += instr[1].parse::<isize>()? * instr[2].parse::<isize>()?;
        }
        Ok(total)
    }

    fn p2(str: Self::Input) -> Result<impl Debug> {
        let pat = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;
        let mut enabled = true;
        let mut total = 0isize;
        for instr in pat.captures_iter(&str.to_str()) {
            if instr[0] == *"do()" {
                enabled = true;
            } else if instr[0] == *"don't()" {
                enabled = false;
            } else if enabled {
                total += instr[1].parse::<isize>()? * instr[2].parse::<isize>()?;
            }
        }
        Ok(total)
    }
}
