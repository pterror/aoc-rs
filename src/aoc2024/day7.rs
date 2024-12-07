use anyhow::Result;

use crate::util::{read_lines, to, CollectResult, CollectVec};

fn parse() -> Result<Vec<(usize, Vec<usize>)>> {
    let lines = read_lines!("inputs/aoc2024/day7.txt")?;
    let a = lines
        .iter()
        .map(|line| {
            let vec = line.split(": ").collect_vec();
            Ok((
                vec[0].parse::<usize>()?,
                vec[1].split(" ").map(to::<usize>).collect_result()?,
            ))
        })
        .collect_result()?;
    Ok(a)
}

fn any_sum(a: usize, ops: &Vec<usize>, acc: usize, idx: usize) -> bool {
    if acc > a {
        false
    } else if idx == ops.len() {
        acc == a
    } else {
        let n = ops[idx];
        any_sum(a, ops, acc + n, idx + 1) || any_sum(a, ops, acc * n, idx + 1)
    }
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum(total, &ops, ops[0], 1) {
            sum += total;
        }
    }
    Ok(format!("{sum}"))
}

fn any_sum_2(a: usize, ops: &Vec<usize>, acc: usize, idx: usize) -> bool {
    if acc > a {
        false
    } else if idx == ops.len() {
        acc == a
    } else {
        let n = ops[idx];
        any_sum_2(a, ops, acc + n, idx + 1) || any_sum_2(a, ops, acc * n, idx + 1) || {
            let concat = acc * (10usize.pow(n.ilog10() + 1)) + n;
            any_sum_2(a, ops, concat, idx + 1)
        }
    }
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum_2(total, &ops, ops[0], 1) {
            sum += total;
        }
    }
    Ok(format!("{sum}"))
}
