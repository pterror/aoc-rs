use std::{collections::HashSet, fmt::Debug};

use anyhow::Result;

use crate::util::*;

fn is_loop(
    xs: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    seen: &mut HashSet<(i32, i32, (i32, i32))>,
) -> bool {
    seen.drain();
    let mut i = i;
    let mut j = j;
    let mut dir = (-1, 0);
    loop {
        let c = xs[i as usize][j as usize];
        if c == '#' {
            if seen.contains(&(i, j, dir)) {
                return true;
            }
            seen.insert((i, j, dir));
            i -= dir.0;
            j -= dir.1;
            dir = (dir.1, -dir.0);
        }
        i += dir.0;
        j += dir.1;
        if i < 0 || j < 0 || (i as usize) >= xs.len() || (j as usize) >= xs[0].len() {
            return false;
        }
    }
}

pub struct Day6;

impl Solution for Day6 {
    type Input = Vec<Vec<char>>;

    fn day() -> u8 {
        6
    }

    fn default_input() -> Result<String> {
        read_string!("inputs/aoc2024/day6.txt")
    }

    fn parse(input: &String) -> Result<Self::Input> {
        Ok(input.lines().map(|x| x.chars().collect_vec()).collect_vec())
    }

    fn p1(mut xs: Self::Input) -> Result<impl Debug> {
        let mut i = 0;
        let mut j = 0;
        let mut dir = (-1, 0);
        'outer: for (i_, l) in xs.iter().enumerate() {
            for (j_, c) in l.iter().enumerate() {
                if *c == '^' {
                    i = i_ as isize;
                    j = j_ as isize;
                    break 'outer;
                }
            }
        }
        let mut count = 0;
        loop {
            let c = xs[i as usize][j as usize];
            if c == '#' {
                i -= dir.0;
                j -= dir.1;
                dir = (dir.1, -dir.0);
            } else if c != 'X' {
                count += 1;
                xs[i as usize][j as usize] = 'X';
            }
            i += dir.0;
            j += dir.1;
            if i < 0 || j < 0 || (i as usize) >= xs.len() || (j as usize) >= xs[0].len() {
                break;
            }
        }
        Ok(count)
    }

    fn p2(mut xs: Self::Input) -> Result<impl Debug> {
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;
        'outer: for (i_, l) in xs.iter().enumerate() {
            for (j_, c) in l.iter().enumerate() {
                if *c == '^' {
                    i = i_ as i32;
                    j = j_ as i32;
                    break 'outer;
                }
            }
        }
        let mut i2 = i;
        let mut j2 = j;
        let mut dir = (-1, 0);
        let mut seen = HashSet::<(i32, i32, (i32, i32))>::new();
        loop {
            let c = xs[i2 as usize][j2 as usize];
            if c == '#' {
                i2 -= dir.0;
                j2 -= dir.1;
                dir = (dir.1, -dir.0);
            } else if c != 'X' {
                xs[i2 as usize][j2 as usize] = '#';
                if is_loop(&xs, i, j, &mut seen) {
                    count += 1;
                }
                xs[i2 as usize][j2 as usize] = 'X';
            }
            i2 += dir.0;
            j2 += dir.1;
            if i2 < 0 || j2 < 0 || (i2 as usize) >= xs.len() || (j2 as usize) >= xs[0].len() {
                break;
            }
        }
        Ok(count)
    }
}
