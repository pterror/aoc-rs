use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn get(xs: &Vec<Vec<u8>>, i: usize, di: isize, j: usize, dj: isize) -> Option<u8> {
    let line = xs.get(((i as isize) + di) as usize)?;
    Some(*line.get(((j as isize) + dj) as usize)?)
}

pub struct Day4;

impl Solution for Day4 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        4
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day4.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for (i, line) in xs.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c != b'X' {
                    continue;
                }
                if get(&xs, i, 0, j, 1) == Some(b'M')
                    && get(&xs, i, 0, j, 2) == Some(b'A')
                    && get(&xs, i, 0, j, 3) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, 0, j, -1) == Some(b'M')
                    && get(&xs, i, 0, j, -2) == Some(b'A')
                    && get(&xs, i, 0, j, -3) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, 0) == Some(b'M')
                    && get(&xs, i, 2, j, 0) == Some(b'A')
                    && get(&xs, i, 3, j, 0) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, 0) == Some(b'M')
                    && get(&xs, i, -2, j, 0) == Some(b'A')
                    && get(&xs, i, -3, j, 0) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, 1) == Some(b'M')
                    && get(&xs, i, 2, j, 2) == Some(b'A')
                    && get(&xs, i, 3, j, 3) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, -1) == Some(b'M')
                    && get(&xs, i, 2, j, -2) == Some(b'A')
                    && get(&xs, i, 3, j, -3) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, 1) == Some(b'M')
                    && get(&xs, i, -2, j, 2) == Some(b'A')
                    && get(&xs, i, -3, j, 3) == Some(b'S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, -1) == Some(b'M')
                    && get(&xs, i, -2, j, -2) == Some(b'A')
                    && get(&xs, i, -3, j, -3) == Some(b'S')
                {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for (i, line) in xs.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c != b'A' {
                    continue;
                }
                let a = get(&xs, i, -1, j, -1);
                let b = get(&xs, i, -1, j, 1);
                let c = get(&xs, i, 1, j, 1);
                let d = get(&xs, i, 1, j, -1);
                if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
                    if a == b'M' && b == b'M' && c == b'S' && d == b'S'
                        || a == b'S' && b == b'M' && c == b'M' && d == b'S'
                        || a == b'S' && b == b'S' && c == b'M' && d == b'M'
                        || a == b'M' && b == b'S' && c == b'S' && d == b'M'
                    {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}
