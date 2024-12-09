use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn any_sum(a: usize, ops: &[usize]) -> bool {
    match ops {
        [] => a == 0,
        [n] => a == *n,
        [rest @ .., n] => (a >= *n && any_sum(a - n, rest)) || (a % n == 0 && any_sum(a / n, rest)),
    }
}

fn any_sum_2(a: usize, ops: &[usize]) -> bool {
    match ops {
        [] => a == 0,
        [n] => a == *n,
        [rest @ .., n] => {
            (a >= *n && any_sum_2(a - n, rest)) || (a % n == 0 && any_sum_2(a / n, rest)) || {
                let modulo = 10usize.pow(n.ilog10() + 1);
                a % modulo == *n && any_sum_2(a / modulo, rest)
            }
        }
    }
}

pub struct Day7;

impl Solution for Day7 {
    type Input = Vec<(usize, Vec<usize>)>;

    fn day() -> u8 {
        7
    }

    fn default_input() -> Result<String> {
        read_string!("inputs/aoc2024/day7.txt")
    }

    fn parse(input: &String) -> Result<Self::Input> {
        input
            .lines()
            .map(|line| {
                let vec = line.split(": ").collect_vec();
                Ok((
                    vec[0].parse::<usize>()?,
                    vec[1].split(" ").map(to::<usize>).collect_result()?,
                ))
            })
            .collect_result()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        for (total, ops) in xs {
            if any_sum(total, &ops) {
                sum += total;
            }
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        for (total, ops) in xs {
            if any_sum_2(total, &ops) {
                sum += total;
            }
        }
        Ok(sum)
    }
}
