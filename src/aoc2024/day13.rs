use std::fmt::Debug;

use anyhow::{Error, Result};

use crate::util::*;

fn score(ax: isize, ay: isize, bx: isize, by: isize, px: isize, py: isize) -> isize {
    let a = (by * px - bx * py) / (ax * by - ay * bx);
    let m = (ay * px - ax * py) / (ay * bx - ax * by);
    if a * ax + m * bx == px && a * ay + m * by == py {
        a * 3 + m
    } else {
        0
    }
}

pub struct Day13;

impl Solution for Day13 {
    type Input = Vec<(isize, isize, isize, isize, isize, isize)>;

    fn day() -> u8 {
        13
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day13.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .collect_vec()
            .split(|x| x.len() == 0)
            .map(|x| {
                let (a, i) = isize::try_search(&x[0]);
                let (b, _) = isize::try_search(&x[0][i..]);
                let (c, i) = isize::try_search(&x[1]);
                let (d, _) = isize::try_search(&x[1][i..]);
                let (e, i) = isize::try_search(&x[2]);
                let (f, _) = isize::try_search(&x[2][i..]);
                if let (Some(ax), Some(ay), Some(bx), Some(by), Some(px), Some(py)) =
                    (a, b, c, d, e, f)
                {
                    Ok((ax, ay, bx, by, px, py))
                } else {
                    Err(Error::msg("day 13: could not parse integers"))
                }
            })
            .collect_result()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        xs.iter()
            .map(|&(ax, ay, bx, by, px, py)| score(ax, ay, bx, by, px, py))
            .sum::<isize>()
            .ok()
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        xs.iter()
            .map(|&(ax, ay, bx, by, px, py)| {
                score(ax, ay, bx, by, px + 10000000000000, py + 10000000000000)
            })
            .sum::<isize>()
            .ok()
    }
}
