use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

use anyhow::{Error, Result};

use crate::util::*;

pub struct Day16;

impl Solution for Day16 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        16
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day16.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut si = 0;
        let mut sj = 0;
        let mut ei = 0;
        let mut ej = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'S' {
                    si = i;
                    sj = j;
                } else if c == b'E' {
                    ei = i;
                    ej = j;
                }
            }
        }
        let mut queue = BinaryHeap::new();
        queue.push((0, (si, sj), (0i8, 1i8)));
        let si = xs.len();
        let sj = xs[0].len();
        let mut seen = HashSet::new();
        while let Some((score, (ni, nj), (di, dj))) = queue.pop() {
            seen.insert(((ni, nj), (di, dj)));
            let left = (-dj, di);
            if !seen.contains(&((ni, nj), left)) {
                queue.push((score - 1000, (ni, nj), left));
            }
            let right = (dj, -di);
            if !seen.contains(&((ni, nj), right)) {
                queue.push((score - 1000, (ni, nj), right));
            }
            if let (Some(ni), Some(nj)) = (
                ni.checked_add_signed(di as isize),
                nj.checked_add_signed(dj as isize),
            ) {
                if ni >= si || nj >= sj {
                    continue;
                }
                if ni == ei && nj == ej {
                    return Ok(-score + 1);
                }
                if xs[ni][nj] != b'#' && !seen.contains(&((ni, nj), (di, dj))) {
                    queue.push((score - 1, (ni, nj), (di, dj)));
                }
            }
        }
        Err(Error::msg("day16: no paths found"))
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut si = 0;
        let mut sj = 0;
        let mut ei = 0;
        let mut ej = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'S' {
                    si = i;
                    sj = j;
                } else if c == b'E' {
                    ei = i;
                    ej = j;
                }
            }
        }
        let mut queue = BinaryHeap::new();
        queue.push((0i32, (si, sj), (0i8, 1i8)));
        let si = xs.len();
        let sj = xs[0].len();
        let mut seen = HashSet::new();
        let mut best = HashMap::new();
        let mut prevs = HashMap::<(usize, usize, i32), HashSet<(usize, usize, i32)>>::new();
        let mut min_score = None;
        while let Some((score, (ni, nj), (di, dj))) = queue.pop() {
            if let Some(min_score_2) = min_score {
                if score < min_score_2 {
                    break;
                }
            }
            seen.insert(((ni, nj), (di, dj)));
            best.insert((ni, nj), score);
            let left = (-dj, di);
            if !seen.contains(&((ni, nj), left)) {
                prevs
                    .entry((ni, nj, score - 1000))
                    .or_default()
                    .insert((ni, nj, score));
                queue.push((score - 1000, (ni, nj), left));
            }
            let right = (dj, -di);
            if !seen.contains(&((ni, nj), right)) {
                prevs
                    .entry((ni, nj, score - 1000))
                    .or_default()
                    .insert((ni, nj, score));
                queue.push((score - 1000, (ni, nj), right));
            }
            if let (Some(ni2), Some(nj2)) = (
                ni.checked_add_signed(di as isize),
                nj.checked_add_signed(dj as isize),
            ) {
                if ni2 >= si || nj2 >= sj {
                    continue;
                }
                if xs[ni2][nj2] != b'#' {
                    prevs
                        .entry((ni2, nj2, score - 1))
                        .or_default()
                        .insert((ni, nj, score));
                    queue.push((score - 1, (ni2, nj2), (di, dj)));
                }
                if ni2 == ei && nj2 == ej {
                    if min_score == None {
                        min_score = Some(score - 1);
                    }
                }
            }
        }
        let mut tiles = HashSet::new();
        add_all_paths_tiles(
            ei,
            ej,
            min_score.unwrap(),
            &prevs,
            &mut HashSet::new(),
            &mut tiles,
        );
        Ok(tiles.len())
    }
}

fn add_all_paths_tiles(
    i: usize,
    j: usize,
    score: i32,
    prevs: &HashMap<(usize, usize, i32), HashSet<(usize, usize, i32)>>,
    seen: &mut HashSet<(usize, usize, i32)>,
    tiles: &mut HashSet<(usize, usize)>,
) {
    seen.insert((i, j, score));
    tiles.insert((i, j));
    for &prev in prevs.get(&(i, j, score)).iter() {
        for &(pi, pj, score) in prev {
            if !seen.contains(&(pi, pj, score)) {
                add_all_paths_tiles(pi, pj, score, prevs, seen, tiles);
            }
        }
    }
}
