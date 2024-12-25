use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day25;

impl Solution for Day25 {
    type Input = (Vec<Vec<u8>>, Vec<Vec<u8>>);

    fn day() -> u8 {
        25
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day25.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let grids = lines
            .split(|l| l.len() == 0)
            .map(|x| x.to_owned())
            .collect_vec();
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for grid in grids {
            if grid[0][0] == b'#' {
                let mut xs = grid[0].iter().map(|_| 0).collect_vec();
                for (i, line) in grid.iter().enumerate() {
                    for (j, &c) in line.iter().enumerate() {
                        if xs[j] == 0 && c == b'.' {
                            xs[j] = i as u8;
                        }
                    }
                }
                locks.push(xs);
            } else {
                let mut xs = grid[0].iter().map(|_| 0).collect_vec();
                for (i, line) in grid.iter().enumerate() {
                    for (j, &c) in line.iter().enumerate() {
                        if xs[j] == 0 && c == b'#' {
                            xs[j] = (7 - i) as u8;
                        }
                    }
                }
                keys.push(xs);
            }
        }
        Ok((locks, keys))
    }

    fn p1((keys, locks): Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for key in keys.iter() {
            for lock in locks.iter() {
                if key.iter().zip(lock).all(|(a, b)| a + b <= 7) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    fn p2(_: Self::Input) -> Result<impl Debug> {
        Ok(":3")
    }
}
