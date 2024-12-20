use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day20;

impl Solution for Day20 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        20
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day20.txt")
        //         r"###############
        // #...#...#.....#
        // #.#.#.#.#.###.#
        // #S#...#.#.#...#
        // #######.#.#.###
        // #######.#.#...#
        // #######.#.###.#
        // ###..E#...#...#
        // ###.#######.###
        // #...###...#...#
        // #.#####.#.###.#
        // #.#...#.#.#...#
        // #.#.#.#.#.#.###
        // #...#...#...###
        // ###############"
        //         r"#################################
        // #S#...#...#####################E#
        // #.#.#.#.#.#####################.#
        // #...#...#.......................#
        // #################################"
        //             .as_bytes()
        //             .to_owned()
        //             .ok()
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut best = xs
            .iter()
            .map(|l| l.iter().map(|_| 0i16).collect_vec())
            .collect_vec();
        let mut sti = 0;
        let mut stj = 0;
        let mut ei = 0;
        let mut ej = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'S' {
                    sti = i;
                    stj = j;
                } else if c == b'E' {
                    ei = i;
                    ej = j;
                }
            }
        }
        let sti = sti;
        let stj = stj;
        let ei = ei;
        let ej = ej;
        let si = xs.len();
        let sj = xs[0].len();
        let mut heap = BinaryHeap::new();
        // -score, i, j
        heap.push((0, sti, stj));
        best[sti][stj] = 0;
        'outer: while let Some((ns, ni, nj)) = heap.pop() {
            for (di, dj) in vec![(0, -1), (-1, 0), (0, 1), (1, 0)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if best[ni2][nj2] != 0 || ni2 == sti && nj2 == stj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    best[ni2][nj2] = -ns + 1;
                    if ni2 == ei && nj2 == ej {
                        break 'outer;
                    } else {
                        heap.push((ns - 1, ni2, nj2));
                    }
                }
            }
        }
        let mut seen = xs
            .iter()
            .map(|l| l.iter().map(|_| false).collect_vec())
            .collect_vec();
        let mut count = 0;
        heap.push((0, sti, stj));
        seen[sti][stj] = true;
        // let mut map = HashMap::new();
        'outer: while let Some((ns, ni, nj)) = heap.pop() {
            for (di, dj) in vec![(0, -2), (-2, 0), (2, 0), (0, 2)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni2 >= si || nj2 >= sj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    let prev_score = best[ni2][nj2];
                    let saved = prev_score - (-ns + 2);
                    if saved <= 0 {
                        continue;
                    }
                    // map.entry(saved)
                    //     .and_modify(|x| {
                    //         *x += 1;
                    //     })
                    //     .or_insert(1);
                    if saved >= 100 {
                        count += 1;
                    }
                }
            }
            for (di, dj) in vec![(0, -1), (-1, 0), (0, 1), (1, 0)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni2 == sti && nj2 == stj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    if ni2 == ei && nj2 == ej {
                        break 'outer;
                    } else {
                        if !seen[ni2][nj2] {
                            seen[ni2][nj2] = true;
                            heap.push((ns - 1, ni2, nj2));
                        }
                    }
                }
            }
        }
        // println!("{map:?}");
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut deltas = Vec::new();
        for di in -20isize..=20 {
            let jr = 20isize - di.abs() as isize;
            for dj in -jr..=jr {
                if di.abs() + dj.abs() > 1 {
                    deltas.push((di, dj));
                }
            }
        }
        let mut best = xs
            .iter()
            .map(|l| l.iter().map(|_| 0i16).collect_vec())
            .collect_vec();
        let mut sti = 0;
        let mut stj = 0;
        let mut ei = 0;
        let mut ej = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'S' {
                    sti = i;
                    stj = j;
                } else if c == b'E' {
                    ei = i;
                    ej = j;
                }
            }
        }
        let sti = sti;
        let stj = stj;
        let ei = ei;
        let ej = ej;
        let si = xs.len();
        let sj = xs[0].len();
        let mut heap = BinaryHeap::new();
        // -score, i, j
        heap.push((0, sti, stj));
        best[sti][stj] = 0;
        'outer: while let Some((ns, ni, nj)) = heap.pop() {
            for (di, dj) in vec![(0, -1), (-1, 0), (0, 1), (1, 0)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if best[ni2][nj2] != 0 || ni2 == sti && nj2 == stj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    best[ni2][nj2] = -ns + 1;
                    if ni2 == ei && nj2 == ej {
                        break 'outer;
                    } else {
                        heap.push((ns - 1, ni2, nj2));
                    }
                }
            }
        }
        let mut seen = xs
            .iter()
            .map(|l| l.iter().map(|_| false).collect_vec())
            .collect_vec();
        let mut count = 0;
        heap.push((0, sti, stj));
        seen[sti][stj] = true;
        let mut map = HashMap::new();
        'outer: while let Some((ns, ni, nj)) = heap.pop() {
            for &(di, dj) in deltas.iter() {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni2 >= si || nj2 >= sj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    let prev_score = best[ni2][nj2];
                    let saved = prev_score - (-ns + di.abs() as i16 + dj.abs() as i16);
                    if saved <= 0 {
                        continue;
                    }
                    // if saved >= 50 {
                    map.entry(saved)
                        .and_modify(|x| {
                            *x += 1;
                        })
                        .or_insert(1);
                    // }
                    if saved >= 100 {
                        count += 1;
                    }
                }
            }
            for (di, dj) in vec![(0, -1), (-1, 0), (0, 1), (1, 0)] {
                if let (Some(ni2), Some(nj2)) =
                    (ni.checked_add_signed(di), nj.checked_add_signed(dj))
                {
                    if ni2 == sti && nj2 == stj {
                        continue;
                    } else if xs[ni2][nj2] == b'#' {
                        continue;
                    }
                    if ni2 == ei && nj2 == ej {
                        break 'outer;
                    } else if seen[ni2][nj2] {
                        continue;
                    }
                    seen[ni2][nj2] = true;
                    heap.push((ns - 1, ni2, nj2));
                }
            }
        }
        // println!("{map:?}");
        Ok(count)
    }
}
