use std::fmt::Debug;

use anyhow::Result;
use regex::Regex;

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
                let pat = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
                let a = x[0].to_str();
                let ma = pat.captures(&a).unwrap();
                let ax = ma[1].parse::<isize>()?;
                let ay = ma[2].parse::<isize>()?;
                let pat = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
                let b = x[1].to_str();
                let mb = pat.captures(&b).unwrap();
                let bx = mb[1].parse::<isize>()?;
                let by = mb[2].parse::<isize>()?;
                let pat = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;
                let c = x[2].to_str();
                let mp = pat.captures(&c).unwrap();
                let px = mp[1].parse::<isize>()?;
                let py = mp[2].parse::<isize>()?;
                Ok((ax, ay, bx, by, px, py))
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
