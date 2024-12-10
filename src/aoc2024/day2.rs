use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn is_part_of_sequence(a: i8, b: i8, sign: i8) -> bool {
    let diff = a - b;
    if diff.signum() != sign {
        return false;
    }
    let abs_diff = diff.abs();
    abs_diff >= 1 && abs_diff <= 3
}

fn valid_with_skips(prev: i8, rest: &[i8], sign: i8, skipped: bool) -> bool {
    if rest.len() == 0 {
        return true;
    }
    let is_rest_match = rest.len() == 1 || is_part_of_sequence(rest[1], rest[0], sign);
    if !is_rest_match && skipped {
        return false;
    }
    if is_rest_match {
        valid_with_skips(rest[0], &rest[1..], sign, skipped)
    } else {
        valid_with_skips(prev, &rest[1..], sign, true)
            || valid_with_skips(rest[0], &rest[2..], sign, true)
    }
}

pub struct Day2;

impl Solution for Day2 {
    type Input = Vec<Vec<i8>>;

    fn day() -> u8 {
        2
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day2.txt")
        // r"80 82 81 82 83 85 88".as_bytes().to_owned().ok()
        //   ^^
        //      XX
        //   XX
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
            let sign = (x[x.len() - 1] - x[0]).signum();
            let mut prev = x[0];
            for &n in x.iter().skip(1) {
                if !is_part_of_sequence(n, prev, sign) {
                    continue 'outer;
                }
                prev = n;
            }
            count += 1;
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for x in xs {
            let sign = (x[x.len() - 1] - x[0]).signum();
            let is_first_valid = is_part_of_sequence(x[1], x[0], sign);
            let is_valid = if is_first_valid {
                valid_with_skips(x[0], &x[1..], sign, false)
            } else {
                valid_with_skips(x[0], &x[2..], sign, true)
                    || valid_with_skips(x[1], &x[2..], sign, true)
            };
            if is_valid {
                count += 1;
            }
        }
        Ok(count)
    }
}
