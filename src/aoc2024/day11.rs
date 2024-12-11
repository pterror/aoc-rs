use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn stones(n: usize, count: u8, cache: &mut HashMap<(usize, u8), usize>) -> usize {
    let cached = cache.get_key_value(&(n, count));
    if let Some((_, &result)) = cached {
        return result;
    }
    if count == 0 {
        1
    } else {
        let mut sum = 0;
        if n == 0 {
            sum += stones(1, count - 1, cache);
        } else {
            let len = n.ilog10() + 1;
            if len % 2 == 0 {
                let modulo = 10usize.pow(len / 2);
                sum += stones(n / modulo, count - 1, cache);
                sum += stones(n % modulo, count - 1, cache);
            } else {
                sum += stones(n * 2024, count - 1, cache);
            }
        };
        cache.insert((n, count), sum);
        sum
    }
}

pub struct Day11;

impl Solution for Day11 {
    type Input = Vec<usize>;

    fn day() -> u8 {
        11
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day11.txt")
        // "125 17".bytes().to_owned().collect_vec().ok()
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
        let mut cache = HashMap::new();
        for x in xs {
            result += stones(x, 25, &mut cache);
        }
        Ok(result)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut result = 0;
        let mut cache = HashMap::new();
        for x in xs {
            result += stones(x, 75, &mut cache);
        }
        Ok(result)
    }
}
