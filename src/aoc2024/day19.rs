use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use crate::util::*;

pub struct Day19;

impl Solution for Day19 {
    type Input = (Vec<Vec<u8>>, Vec<Vec<u8>>);

    fn day() -> u8 {
        19
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day19.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let a = lines[0]
            .split(|&x| x == b',')
            .map(|x| (if x[0] == b' ' { &x[1..] } else { x }).to_owned())
            .collect_vec();
        let b = lines[2..].to_owned();
        Ok((a, b))
    }

    fn p1((available, expected): Self::Input) -> Result<impl Debug> {
        let mut cache = HashMap::new();
        Ok(expected
            .iter()
            .filter(|e| num_possible(&available, e, &mut cache) != 0)
            .count())
    }

    fn p2((available, expected): Self::Input) -> Result<impl Debug> {
        let mut cache = HashMap::new();
        Ok(expected
            .iter()
            .map(|e| num_possible(&available, e, &mut cache))
            .sum::<usize>())
    }
}

fn num_possible(
    available: &[Vec<u8>],
    expected: &[u8],
    map: &mut HashMap<Vec<u8>, usize>,
) -> usize {
    if let Some(&count) = map.get(&expected.to_vec()) {
        return count;
    }
    if expected.len() == 0 {
        1
    } else {
        let mut count = 0;
        'outer: for a in available {
            if a.len() > expected.len() {
                continue;
            }
            for (i, &c) in a.iter().enumerate() {
                if c != expected[i] {
                    continue 'outer;
                }
            }
            count += num_possible(available, &expected[a.len()..], map);
        }
        map.insert(expected.to_owned(), count);
        count
    }
}
