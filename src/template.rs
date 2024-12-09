use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day0;

impl Solution for Day0 {
    type Input = Vec<u8>;

    fn day() -> u8 {
        0
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day9.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.clone().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }
}
