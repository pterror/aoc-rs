use std::fmt::Debug;

use anyhow::Result;
use regex::Regex;

use crate::util::*;

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
        let mut sum = 0;
        for (ax, ay, bx, by, px, py) in xs {
            let a = (by * px - bx * py) / (ax * by - ay * bx);
            let m = (ay * px - ax * py) / (ay * bx - ax * by);
            if a * ax + m * bx == px && a * ay + m * by == py {
                sum += a * 3 + m;
            }
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        // 10000000000000
        let mut sum = 0;
        for (ax, ay, bx, by, px, py) in xs {
            let px = px + 10000000000000;
            let py = py + 10000000000000;
            // a ax + b bx = px
            // a ay + b by = py
            // a py ax + b py bx = px py
            // a px ay + b px by = px py
            // a py ax + b py bx = a px ay + b px by
            // a (py ax - px ay) = b (px by - py bx)

            // a ax + b bx = px
            // a = (px - b bx) / ax

            // (px - b bx) (py ax - px ay) / ax = b (px by - py bx)
            // px py - b bx py - px2 ay / ax + px2 ay / ax = b (px by - py bx)
            // px py - px2 ay / ax + px2 ay / ax = b (px by - py bx + bx py)
            // px py ax / (ax px by) = b
            let a = (by * px - bx * py) / (ax * by - ay * bx);
            let m = (ay * px - ax * py) / (ay * bx - ax * by);
            if a * ax + m * bx == px && a * ay + m * by == py {
                sum += a * 3 + m;
            }
        }
        Ok(sum)
    }
}
