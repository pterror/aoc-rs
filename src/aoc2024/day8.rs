use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<Vec<char>>> {
    let lines = read_lines!("inputs/aoc2024/day8.txt")?;
    Ok(lines.iter().map(|x| x.chars().collect()).collect())
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut locs = HashMap::<char, HashSet<(usize, usize)>>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            locs.entry(c).or_default().insert((i, j));
        }
    }
    let mut antinodes = HashMap::<char, HashSet<(usize, usize)>>::new();
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    for (c, locs) in locs {
        let an = antinodes.entry(c).or_default();
        for &(ai, aj) in locs.iter() {
            for &(bi, bj) in locs.iter() {
                if ai == bi && aj == bj {
                    continue;
                }
                let di = (ai as isize) - (bi as isize);
                let dj = (aj as isize) - (bj as isize);
                let p1 = ((ai as isize) + di, (aj as isize) + dj);
                let p2 = ((bi as isize) - di, (bj as isize) - dj);
                if p1.0 >= 0 && p1.0 < si && p1.1 >= 0 && p1.1 < sj {
                    an.insert((p1.0 as usize, p1.1 as usize));
                }
                if p2.0 >= 0 && p2.0 < si && p2.1 >= 0 && p2.1 < sj {
                    an.insert((p2.0 as usize, p2.1 as usize));
                }
            }
        }
    }
    let count = antinodes
        .iter()
        .flat_map(|(_, v)| v.iter())
        .collect::<HashSet<_>>()
        .len();
    Ok(format!("{count}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut locs = HashMap::<char, HashSet<(usize, usize)>>::new();
    for (i, l) in xs.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            locs.entry(c).or_default().insert((i, j));
        }
    }
    let mut antinodes = HashMap::<char, HashSet<(usize, usize)>>::new();
    let si = xs.len() as isize;
    let sj = xs[0].len() as isize;
    for (c, locs) in locs {
        let an = antinodes.entry(c).or_default();
        for &(ai, aj) in locs.iter() {
            for &(bi, bj) in locs.iter() {
                if ai == bi && aj == bj {
                    an.insert((ai, aj));
                    continue;
                }
                let di = (ai as isize) - (bi as isize);
                let dj = (aj as isize) - (bj as isize);
                let mut p1 = ((ai as isize) + di, (aj as isize) + dj);
                while p1.0 >= 0 && p1.0 < si && p1.1 >= 0 && p1.1 < sj {
                    an.insert((p1.0 as usize, p1.1 as usize));
                    p1.0 += di;
                    p1.1 += dj;
                }
                let mut p2 = ((bi as isize) - di, (bj as isize) - dj);
                while p2.0 >= 0 && p2.0 < si && p2.1 >= 0 && p2.1 < sj {
                    an.insert((p2.0 as usize, p2.1 as usize));
                    p2.0 -= di;
                    p2.1 -= dj;
                }
            }
        }
    }
    let count = antinodes
        .iter()
        .flat_map(|(_, v)| v.iter())
        .collect::<HashSet<_>>()
        .len();
    Ok(format!("{count}"))
}
