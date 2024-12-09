use std::{cmp::Ordering, collections::HashSet, fmt::Debug};

use anyhow::Result;

use crate::util::*;

pub struct Day5;

impl Solution for Day5 {
    type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);

    fn day() -> u8 {
        5
    }

    fn default_input() -> Result<String> {
        read_string!("inputs/aoc2024/day5.txt")
    }

    fn parse(input: &String) -> Result<Self::Input> {
        let what = input.split("\n\n").collect_vec();
        let a = what[0]
            .lines()
            .map(|l| {
                let x = l.split("|").collect_vec();
                Ok((x[0].parse::<usize>()?, x[1].parse::<usize>()?))
            })
            .collect_result()?;
        let b = what[1]
            .lines()
            .map(|l| l.split(",").map(to::<usize>).collect_result())
            .collect_result()?;
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
        'outer: for book in books {
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
            if book == sorted {
                continue 'outer;
            }
            sum += sorted[sorted.len() / 2];
        }
        Ok(sum)
    }
}
