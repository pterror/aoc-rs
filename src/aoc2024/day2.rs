use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn is_valid(x: &[i8]) -> bool {
    let sign = (x[1] - x[0]).signum();
    for (i, n) in x.iter().skip(1).enumerate() {
        let diff = n - x[i];
        if diff.signum() != sign {
            return false;
        }
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }
    true
}

pub struct Day2;

impl Solution for Day2 {
    type Input = Vec<Vec<i8>>;

    fn day() -> u8 {
        2
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day2.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|line| line.split(|&x| x == b' ').map(|x| x.parse()).collect_vec())
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for x in xs {
            if is_valid(&x) {
                count += 1;
            }
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        'outer: for x in xs {
            for (i, _) in x.iter().enumerate() {
                let x = x
                    .iter()
                    .take(i)
                    .chain(x.iter().skip(i + 1))
                    .map(|x| *x)
                    .collect_vec();
                if is_valid(&x) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        Ok(count)
    }
}
