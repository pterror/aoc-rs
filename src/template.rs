use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day0;

impl Solution for Day0 {
    type Input = String;

    fn day() -> u8 {
        0
    }

    fn default_input() -> Result<String> {
        read_string!("inputs/aoc2024/day9.txt")
    }

    fn parse(input: &String) -> Result<Self::Input> {
        input.clone().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }
}
