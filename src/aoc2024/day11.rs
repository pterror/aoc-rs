use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LazyLock, Mutex};

use anyhow::Result;

use crate::util::*;

static STONES_CACHE: LazyLock<Mutex<HashMap<(usize, u8), usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn stones(n: usize, count: u8) -> usize {
    let cached = {
        let locked = STONES_CACHE.lock().unwrap();
        locked.get(&(n, count)).map(|&x| x)
    };
    if let Some(result) = cached {
        result
    } else if count == 0 {
        1
    } else {
        let sum = if n == 0 {
            stones(1, count - 1)
        } else {
            let len = n.ilog10() + 1;
            if len % 2 == 0 {
                let modulo = 10usize.pow(len / 2);
                stones(n / modulo, count - 1) + stones(n % modulo, count - 1)
            } else {
                stones(n * 2024, count - 1)
            }
        };
        STONES_CACHE.lock().unwrap().insert((n, count), sum);
        sum
    }
}

pub struct Day11;

impl Solution for Day11 {
    type Input = Vec<usize>;

    fn day() -> u8 {
        11
    }

    fn reset_global_state() {
        STONES_CACHE.lock().unwrap().clear();
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day11.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .split(|&x| x == b' ')
            .map(|x| x.parse())
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut result = 0;
        for x in xs {
            result += stones(x, 25);
        }
        Ok(result)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut result = 0;
        for x in xs {
            result += stones(x, 75);
        }
        Ok(result)
    }
}
