use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn get(xs: &Vec<String>, i: usize, di: isize, j: usize, dj: isize) -> Option<char> {
    let line = xs.get(((i as isize) + di) as usize)?;
    Some(line.chars().nth(((j as isize) + dj) as usize)?)
}

pub struct Day4;

impl Solution for Day4 {
    type Input = Vec<String>;

    fn day() -> u8 {
        4
    }

    fn default_input() -> Result<String> {
        read_string!("inputs/aoc2024/day4.txt")
    }

    fn parse(input: &String) -> Result<Self::Input> {
        input.lines().map(|x| x.into()).collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        for (i, line) in xs.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c != 'X' {
                    continue;
                }
                if get(&xs, i, 0, j, 1) == Some('M')
                    && get(&xs, i, 0, j, 2) == Some('A')
                    && get(&xs, i, 0, j, 3) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, 0, j, -1) == Some('M')
                    && get(&xs, i, 0, j, -2) == Some('A')
                    && get(&xs, i, 0, j, -3) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, 0) == Some('M')
                    && get(&xs, i, 2, j, 0) == Some('A')
                    && get(&xs, i, 3, j, 0) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, 0) == Some('M')
                    && get(&xs, i, -2, j, 0) == Some('A')
                    && get(&xs, i, -3, j, 0) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, 1) == Some('M')
                    && get(&xs, i, 2, j, 2) == Some('A')
                    && get(&xs, i, 3, j, 3) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, 1, j, -1) == Some('M')
                    && get(&xs, i, 2, j, -2) == Some('A')
                    && get(&xs, i, 3, j, -3) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, 1) == Some('M')
                    && get(&xs, i, -2, j, 2) == Some('A')
                    && get(&xs, i, -3, j, 3) == Some('S')
                {
                    count += 1;
                }
                if get(&xs, i, -1, j, -1) == Some('M')
                    && get(&xs, i, -2, j, -2) == Some('A')
                    && get(&xs, i, -3, j, -3) == Some('S')
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
            for (j, c) in line.chars().enumerate() {
                if c != 'A' {
                    continue;
                }
                let a = get(&xs, i, -1, j, -1);
                let b = get(&xs, i, -1, j, 1);
                let c = get(&xs, i, 1, j, 1);
                let d = get(&xs, i, 1, j, -1);
                if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
                    if a == 'M' && b == 'M' && c == 'S' && d == 'S'
                        || a == 'S' && b == 'M' && c == 'M' && d == 'S'
                        || a == 'S' && b == 'S' && c == 'M' && d == 'M'
                        || a == 'M' && b == 'S' && c == 'S' && d == 'M'
                    {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}
