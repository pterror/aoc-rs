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
                        // println!(
                        //     "{}",
                        //     map.iter()
                        //         .enumerate()
                        //         .map(|(i, l)| {
                        //             let bytes = l
                        //                 .iter()
                        //                 .enumerate()
                        //                 .map(|(j, &b)| {
                        //                     if b {
                        //                         b'#'
                        //                     } else {
                        //                         if (i == 0 && j == 0) || scores[i][j] != 0 {
                        //                             b'O'
                        //                         } else {
                        //                             b'.'
                        //                         }
                        //                     }
                        //                 })
                        //                 .collect_vec();
                        //             String::from_utf8_lossy(&bytes).to_string()
                        //         })
                        //         .collect_vec()
                        //         .join("\n")
                        // );
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
        let mut map = vec![vec![false; si]; sj];
        let mut heap = BinaryHeap::<(i16, usize, usize)>::new();
        'outer: for &(i, j) in xs.iter() {
            heap.drain();
            let mut scores = vec![vec![0i16; si]; sj];
            map[i as usize][j as usize] = true;
            // steps, i, j
            heap.push((0, 0, 0));
            while let Some((ns, ni, nj)) = heap.pop() {
                for (di, dj) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    if let (Some(ni2), Some(nj2)) =
                        (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                    {
                        if ni == si - 1 && nj == sj - 1 {
                            continue 'outer;
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
            return Ok(format!("{i},{j}"));
        }
        Err(Error::msg("day18 part 2: no solution"))
    }
}
