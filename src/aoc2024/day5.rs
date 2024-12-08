use std::{cmp::Ordering, collections::HashSet};

use anyhow::Result;

use crate::util::{read_string, to, CollectResult, CollectVec};

fn parse() -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let file = read_string!("inputs/aoc2024/day5.txt")?;
    let what = file.split("\n\n").collect_vec();
    let a = what[0];
    let b = what[1];
    let a = a
        .lines()
        .map(|l| {
            let x = l.split("|").collect_vec();
            Ok((x[0].parse::<usize>()?, x[1].parse::<usize>()?))
        })
        .collect_result()?;
    let b = b
        .lines()
        .map(|l| l.split(",").map(to::<usize>).collect_result())
        .collect_result()?;
    Ok((a, b))
}

pub fn p1() -> Result<String> {
    let (rules, books) = parse()?;
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
    Ok(format!("{sum}"))
}

pub fn p2() -> Result<String> {
    let (rules, books) = parse()?;
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
    Ok(format!("{sum}"))
}
