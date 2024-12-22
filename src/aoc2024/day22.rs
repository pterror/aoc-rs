use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use anyhow::Result;

use crate::util::*;

pub struct Day22;

impl Solution for Day22 {
    type Input = Vec<isize>;

    fn day() -> u8 {
        22
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day22.txt")
        //         br"1
        // 2
        // 3
        // 2024"
        //             .to_vec()
        //             .ok()
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|x| x.as_slice().parse::<isize>())
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        for x in xs {
            sum += steps(x, 2000);
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut sums = HashMap::new();
        let mut max_value = 0;
        for x in xs {
            let mut a = x;
            let mut b = step(a);
            let mut c = step(b);
            let mut d = step(c);
            let mut e = step(d);
            let mut done = HashSet::new();
            for _ in 0..1996 {
                let (da, db, dc, dd) = (
                    (b % 10) - (a % 10),
                    (c % 10) - (b % 10),
                    (d % 10) - (c % 10),
                    (e % 10) - (d % 10),
                );
                let deltas = (da, db, dc, dd);
                if !done.contains(&deltas) {
                    done.insert(deltas);
                    let new = e % 10;
                    let value = sums
                        .entry((da, db, dc, dd))
                        .and_modify(|prev| *prev = *prev + new)
                        .or_insert(new);
                    if *value > max_value {
                        max_value = *value;
                    }
                }
                (a, b, c, d, e) = (b, c, d, e, step(e));
            }
        }
        Ok(max_value)
    }
}

fn steps(mut x: isize, steps: usize) -> isize {
    for _ in 0..steps {
        x = step(x);
    }
    x
}

fn step(mut x: isize) -> isize {
    let a = x * 64;
    x = (x ^ a) % 16777216;
    let b = x / 32;
    x = (x ^ b) % 16777216;
    let c = x * 2048;
    x = (x ^ c) % 16777216;
    x
}
