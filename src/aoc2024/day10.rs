use std::{collections::HashSet, fmt::Debug};

use anyhow::Result;

use crate::util::*;

pub struct Day10;

fn count_peaks(
    xs: &Vec<Vec<u8>>,
    i: usize,
    j: usize,
    h: u8,
    seen: &mut Option<HashSet<(usize, usize)>>,
) -> usize {
    let si = xs.len();
    let sj = xs[0].len();
    let positions = vec![(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(|&(di, dj)| (i.checked_add_signed(di), j.checked_add_signed(dj)))
        .flat_map(|(i, j)| {
            if let (Some(i), Some(j)) = (i, j) {
                if i < si && j < sj {
                    Some((i, j))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec();
    let mut count = 0;
    for &pos @ (i2, j2) in positions.iter() {
        let h2 = xs[i2][j2];
        if h2 == h + 1 {
            if h2 == 9 {
                let seen_before = seen.as_ref().map_or(true, |seen| !seen.contains(&pos));
                if seen_before {
                    if let Some(seen) = seen {
                        seen.insert(pos);
                    }
                    count += 1;
                }
            } else {
                count += count_peaks(xs, i2, j2, h + 1, seen);
            }
        }
    }
    count
}

impl Solution for Day10 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        10
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day10.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|x| x.iter().map(|c| c - b'0').collect_vec())
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == 0 {
                    let new = count_peaks(&xs, i, j, 0, &mut Some(HashSet::new()));
                    count += new;
                }
            }
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == 0 {
                    count += count_peaks(&xs, i, j, 0, &mut None);
                }
            }
        }
        Ok(count)
    }
}
