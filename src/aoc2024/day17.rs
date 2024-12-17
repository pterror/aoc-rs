use std::{collections::HashSet, fmt::Debug};

use anyhow::{Error, Result};

use crate::util::*;

pub struct Day17;

impl Solution for Day17 {
    type Input = (usize, usize, usize, Vec<u8>);

    fn day() -> u8 {
        17
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day17.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let (a, _) = usize::try_search(&lines[0]);
        let (b, _) = usize::try_search(&lines[1]);
        let (c, _) = usize::try_search(&lines[2]);
        let x = lines[4]["Program: ".len()..]
            .split(|&x| x == b',')
            .map(|x| x.parse::<u8>())
            .collect_vec();
        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            Ok((a, b, c, x))
        } else {
            Err(Error::msg("day 17: invalid input"))
        }
    }

    fn p1((a, b, c, xs): Self::Input) -> Result<impl Debug> {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut out = Vec::new();
        let mut i = 0;
        while i < xs.len() {
            match xs[i] {
                0 => {
                    a = a >> read_combo_operand(xs[i + 1], a, b, c);
                }
                1 => {
                    b ^= xs[i + 1] as usize;
                }
                2 => {
                    b = read_combo_operand(xs[i + 1], a, b, c) % 8;
                }
                3 => {
                    if a != 0 {
                        i = xs[i + 1] as usize;
                        continue;
                    }
                }
                4 => {
                    b ^= c;
                }
                5 => {
                    out.push(read_combo_operand(xs[i + 1], a, b, c) % 8);
                }
                6 => {
                    b = a >> read_combo_operand(xs[i + 1], a, b, c);
                }
                7 => {
                    c = a >> read_combo_operand(xs[i + 1], a, b, c);
                }
                _ => {}
            }
            i += 2;
        }
        Ok(out.iter().map(|x| x.to_string()).collect_vec().join(","))
    }

    fn p2((_, _, _, xs): Self::Input) -> Result<impl Debug> {
        // 5034303085 10 2
        // 5034303167 10 2
        // 5302738541 10 2
        // 5302738623 10 2
        // 6108044909 10 2
        // 6108044991 10 2
        // 6376480365 10 2
        // 6376480447 10 2
        let mut start = 0;
        let mut step = 0;
        for cand in 0.. {
            let cand: usize = cand * 1073741824 + 5302738541;
            let mut a = cand;
            let mut b = 0;
            let mut c = 0;
            let mut i = 0;
            let mut out_i = 0;
            while i < xs.len() && out_i <= xs.len() {
                match xs[i] {
                    0 => {
                        a = a >> read_combo_operand(xs[i + 1], a, b, c);
                    }
                    1 => {
                        b ^= xs[i + 1] as usize;
                    }
                    2 => {
                        b = read_combo_operand(xs[i + 1], a, b, c) % 8;
                    }
                    3 => {
                        if a != 0 {
                            i = xs[i + 1] as usize;
                            continue;
                        }
                    }
                    4 => {
                        b ^= c;
                    }
                    5 => {
                        let next = read_combo_operand(xs[i + 1], a, b, c) % 8;
                        if next as u8 != xs[out_i] {
                            break;
                        }
                        out_i += 1;
                    }
                    6 => {
                        b = a >> read_combo_operand(xs[i + 1], a, b, c);
                    }
                    7 => {
                        c = a >> read_combo_operand(xs[i + 1], a, b, c);
                    }
                    _ => {}
                }
                i += 2;
            }
            if out_i >= 12 {
                if start == 0 {
                    start = cand;
                } else if step == 0 {
                    step = cand - start;
                } else {
                    step = gcd(cand - start, step);
                }
                // println!("{cand} {out_i} {step} {}");
            }
            if out_i == xs.len() {
                return Ok(cand);
            }
        }
        Err(Error::msg("day 17 part 2: unreachable"))
    }
}

fn read_combo_operand(op: u8, a: usize, b: usize, c: usize) -> usize {
    match op {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => 0,
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else if b > a {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}
