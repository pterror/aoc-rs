use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<Vec<char>>> {
    let lines = read_lines!("inputs/aoc2024/day8.txt")?;
    Ok(lines.iter().map(|x| x.chars().collect()).collect())
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut locs = HashMap::<char, HashSet<(isize, isize)>>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            locs.entry(c).or_default().insert((i as isize, j as isize));
        }
    }
    let mut an = HashSet::<(isize, isize)>::new();
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    for (_, locs) in locs {
        for &(ai, aj) in locs.iter() {
            for &(bi, bj) in locs.iter() {
                if ai == bi && aj == bj {
                    continue;
                }
                let di = bi - ai;
                let dj = bj - aj;
                let i1 = ai - di;
                let j1 = aj - dj;
                if i1 >= 0 && i1 < si && j1 >= 0 && j1 < sj {
                    an.insert((i1, j1));
                }
                let i2 = bi + di;
                let j2 = bj + dj;
                if i2 >= 0 && i2 < si && j2 >= 0 && j2 < sj {
                    an.insert((i2, j2));
                }
            }
        }
    }
    let count = an.len();
    Ok(format!("{count}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut locs = HashMap::<char, HashSet<(isize, isize)>>::new();
    let mut an = HashSet::<(isize, isize)>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            let p = (i as isize, j as isize);
            locs.entry(c).or_default().insert(p);
            an.insert(p);
        }
    }
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    for (_, locs) in locs {
        for &(ai, aj) in locs.iter() {
            for &(bi, bj) in locs.iter() {
                if ai == bi && aj == bj {
                    continue;
                }
                let di = ai - bi;
                let dj = aj - bj;
                let mut i1 = ai + di;
                let mut j1 = aj + dj;
                while i1 >= 0 && i1 < si && j1 >= 0 && j1 < sj {
                    an.insert((i1, j1));
                    i1 += di;
                    j1 += dj;
                }
                let mut i2 = bi - di;
                let mut j2 = bj - dj;
                while i2 >= 0 && i2 < si && j2 >= 0 && j2 < sj {
                    an.insert((i2, j2));
                    i2 -= di;
                    j2 -= dj;
                }
            }
        }
    }
    let count = an.len();
    Ok(format!("{count}"))
}
