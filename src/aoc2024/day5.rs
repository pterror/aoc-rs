use std::{cmp::Ordering, collections::HashSet, fmt::Debug};

use anyhow::Result;

use crate::util::*;

pub struct Day5;

impl Solution for Day5 {
    type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);

    fn day() -> u8 {
        5
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day5.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let input = input.lines().collect_vec();
        let what = input.split(|x| x.len() == 0).collect_vec();
        let a = what[0]
            .iter()
            .map(|l| {
                let x = l.split(|&x| x == b'|').collect_vec();
                (x[0].parse(), x[1].parse())
            })
            .collect_vec();
        let b = what[1]
            .iter()
            .map(|l| l.split(|&x| x == b',').map(|x| x.parse()).collect_vec())
            .collect_vec();
        Ok((a, b))
    }

    fn p1((rules, books): Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        'outer: for book in books {
            for (i, x) in book.iter().enumerate() {
                for (_, b) in rules.iter().filter(|(a, _)| x == a) {
                    if book[0..i].contains(b) {
                        continue 'outer;
                    }
                }
            }
            sum += book[book.len() / 2];
        }
        Ok(sum)
    }

    fn p2((rules, books): Self::Input) -> Result<impl Debug> {
        let rules_map = rules.iter().collect::<HashSet<_>>();
        let mut sum = 0;
        for book in books {
            let mut sorted = book.clone();
            sorted.sort_by(|&a, &b| {
                if rules_map.contains(&(a, b)) {
                    return Ordering::Less;
                } else if rules_map.contains(&(b, a)) {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            });
            if book != sorted {
                sum += sorted[sorted.len() / 2];
            }
        }
        Ok(sum)
    }
}
