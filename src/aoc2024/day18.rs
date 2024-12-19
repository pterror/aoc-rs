use std::{collections::BinaryHeap, fmt::Debug};

use anyhow::{Error, Result};

use crate::util::*;

pub struct Day18;

impl Solution for Day18 {
    type Input = Vec<(u8, u8)>;

    fn day() -> u8 {
        18
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day18.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|x| {
                let a = x.split(|&c| c == b',').collect_vec();
                (a[0].parse(), a[1].parse())
            })
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let si = 71;
        let sj = 71;
        let mut map = vec![vec![false; si]; sj];
        let mut scores = vec![vec![0i16; si]; sj];
        for &(i, j) in xs.iter().take(1024) {
            map[i as usize][j as usize] = true;
        }
        let mut heap = BinaryHeap::<(i16, usize, usize)>::new();
        // steps, i, j
        heap.push((0, 0, 0));
        while let Some((ns, ni, nj)) = heap.pop() {
            for (di, dj) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni == si - 1 && nj == sj - 1 {
                        return Ok(-ns);
                    }
                    if ni2 < si && nj2 < sj {
                        if (ni2 != 0 || nj2 != 0) && scores[ni2][nj2] == 0 && !map[ni2][nj2] {
                            scores[ni2][nj2] = ns - 1;
                            heap.push((ns - 1, ni2, nj2));
                        }
                    }
                }
            }
        }
        Err(Error::msg("day18 part 1: no solution"))
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let si = 71;
        let sj = 71;
        let mut heap = BinaryHeap::<(u16, usize, usize)>::new();
        let last = xs.len() as u16;
        let mut map = vec![vec![last; si]; sj];
        let mut seen = vec![vec![false; si]; sj];
        for (idx, &(i, j)) in xs.iter().enumerate() {
            map[i as usize][j as usize] = idx as u16;
        }
        seen[0][0] = true;
        let mut idx = xs.len() as u16;
        // steps, i, j
        heap.push((map[0][0], 0, 0));
        while let Some((ns, ni, nj)) = heap.pop() {
            idx = idx.min(ns);
            for (di, dj) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni == si - 1 && nj == sj - 1 {
                        let (i, j) = xs[idx as usize];
                        return Ok(format!("{i},{j}"));
                    }
                    if ni2 < si && nj2 < sj {
                        if !seen[ni2][nj2] {
                            seen[ni2][nj2] = true;
                            heap.push((map[ni2][nj2], ni2, nj2));
                        }
                    }
                }
            }
        }
        Err(Error::msg("day18 part 2: no solution"))
    }
}
