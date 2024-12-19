use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day1;

impl Solution for Day1 {
    type Input = (Vec<usize>, Vec<usize>);

    fn day() -> u8 {
        1
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day1.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for line in input.lines() {
            let nums = line.split(|&x| x == b' ').collect_vec();
            a.push(nums[0].parse());
            b.push(nums[nums.len() - 1].parse());
        }
        Ok((a, b))
    }

    fn p1((mut a, mut b): Self::Input) -> Result<impl Debug> {
        a.sort();
        b.sort();
        a.iter()
            .zip(b)
            .map(|(a, b)| a.abs_diff(b))
            .sum::<usize>()
            .ok()
    }

    fn p2((a, b): Self::Input) -> Result<impl Debug> {
        a.iter()
            .map(|&a| a * b.iter().filter(|&&b| a == b).count())
            .sum::<usize>()
            .ok()
    }
}
