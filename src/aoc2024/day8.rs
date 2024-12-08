use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<Vec<char>>> {
    let lines = read_lines!("inputs/aoc2024/day8.txt")?;
    Ok(lines.iter().map(|x| x.chars().collect()).collect())
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    let mut locs = HashMap::<char, Vec<(isize, isize)>>::new();
    let mut an = HashSet::<(isize, isize)>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            if c == '.' {
                continue;
            }
            let others = locs.entry(c).or_default();
            for &(ai, aj) in others.iter() {
                let di = i - ai;
                let dj = j - aj;
                let i1 = ai - di;
                let j1 = aj - dj;
                if i1 >= 0 && i1 < si && j1 >= 0 && j1 < sj {
                    an.insert((i1, j1));
                }
                let i2 = i + di;
                let j2 = j + dj;
                if i2 >= 0 && i2 < si && j2 >= 0 && j2 < sj {
                    an.insert((i2, j2));
                }
            }
            others.push((i, j));
        }
    }
    let count = an.len();
    Ok(format!("{count}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    let mut locs = HashMap::<char, Vec<(isize, isize)>>::new();
    let mut an = HashSet::<(isize, isize)>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            if c == '.' {
                continue;
            }
            let p = (i, j);
            an.insert(p);
            let others = locs.entry(c).or_default();
            for &(ai, aj) in others.iter() {
                let di = ai - i;
                let dj = aj - j;
                let mut i1 = ai + di;
                let mut j1 = aj + dj;
                while i1 >= 0 && i1 < si && j1 >= 0 && j1 < sj {
                    an.insert((i1, j1));
                    i1 += di;
                    j1 += dj;
                }
                let mut i2 = i - di;
                let mut j2 = j - dj;
                while i2 >= 0 && i2 < si && j2 >= 0 && j2 < sj {
                    an.insert((i2, j2));
                    i2 -= di;
                    j2 -= dj;
                }
            }
            others.push(p);
        }
    }
    let count = an.len();
    Ok(format!("{count}"))
}
