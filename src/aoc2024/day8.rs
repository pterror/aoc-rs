use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::util::read_byte_lines;

fn parse() -> Result<Vec<Vec<u8>>> {
    read_byte_lines!("inputs/aoc2024/day8.txt")
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut ans = xs.clone();
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    let mut locs = HashMap::<u8, Vec<(isize, isize)>>::new();
    let mut count = 0;
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            if c == b'.' {
                continue;
            }
            let others = locs.entry(c).or_default();
            for &(ai, aj) in others.iter() {
                let di = i - ai;
                let dj = j - aj;
                {
                    let i = ai - di;
                    let j = aj - dj;
                    if i >= 0 && i < si && j >= 0 && j < sj && ans[i as usize][j as usize] != b'#' {
                        ans[i as usize][j as usize] = b'#';
                        count += 1;
                    }
                }
                {
                    let i = i + di;
                    let j = j + dj;
                    if i >= 0 && i < si && j >= 0 && j < sj && ans[i as usize][j as usize] != b'#' {
                        ans[i as usize][j as usize] = b'#';
                        count += 1;
                    }
                }
            }
            others.push((i, j));
        }
    }
    Ok(format!("{count}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut ans = xs.clone();
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    let mut locs = HashMap::<u8, Vec<(isize, isize)>>::new();
    let mut an = HashSet::<(isize, isize)>::new();
    let mut count = 0;
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            if c == b'.' {
                continue;
            }
            let p = (i, j);
            an.insert(p);
            let others = locs.entry(c).or_default();
            if ans[i as usize][j as usize] != b'#' {
                count += 1;
                ans[i as usize][j as usize] = b'#';
            }
            for &(ai, aj) in others.iter() {
                let di = ai - i;
                let dj = aj - j;
                {
                    let mut i = ai + di;
                    let mut j = aj + dj;
                    while i >= 0 && i < si && j >= 0 && j < sj {
                        if ans[i as usize][j as usize] != b'#' {
                            count += 1;
                            ans[i as usize][j as usize] = b'#';
                        }
                        i += di;
                        j += dj;
                    }
                }
                {
                    let mut i = i - di;
                    let mut j = j - dj;
                    while i >= 0 && i < si && j >= 0 && j < sj {
                        if ans[i as usize][j as usize] != b'#' {
                            count += 1;
                            ans[i as usize][j as usize] = b'#';
                        }
                        i -= di;
                        j -= dj;
                    }
                }
            }
            others.push(p);
        }
    }
    Ok(format!("{count}"))
}
