use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

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
        'outer: for x in xs {
            let sign = (x[1] - x[0]).signum();
            for (i, n) in x.iter().skip(1).enumerate() {
                let diff = n - x[i];
                if diff.signum() != sign {
                    continue 'outer;
                }
                let abs_diff = diff.abs();
                if abs_diff < 1 || abs_diff > 3 {
                    continue 'outer;
                }
            }
            count += 1;
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        'outer: for x in xs {
            for (i, _) in x.iter().enumerate() {
                let x = x.iter().take(i).chain(x.iter().skip(i + 1)).collect_vec();
                let diffs = x
                    .iter()
                    .skip(1)
                    .enumerate()
                    .map(|(i, n)| *n - x[i])
                    .collect_vec();
                if !diffs.iter().all(|n| *n < 0) && !diffs.iter().all(|n| *n > 0) {
                    continue;
                }
                if !diffs.iter().all(|n| n.abs() >= 1 && n.abs() <= 3) {
                    continue;
                }
                count += 1;
                continue 'outer;
            }
        }
        Ok(count)
    }
}
