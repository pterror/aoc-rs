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

pub fn p1() -> Result<()> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum(total, &ops, ops[0], 1) {
            sum += total;
        }
    }
    println!("{sum}");
    Ok(())
}

fn any_sum_2(a: usize, ops: &Vec<usize>, acc: usize, idx: usize) -> Result<bool> {
    if acc > a {
        Ok(false)
    } else if idx == ops.len() {
        Ok(acc == a)
    } else {
        let n = ops[idx];
        Ok(any_sum_2(a, ops, acc + n, idx + 1)?
            || any_sum_2(a, ops, acc * n, idx + 1)?
            || any_sum_2(a, ops, format!("{acc}{n}").parse::<usize>()?, idx + 1)?)
    }
}

pub fn p2() -> Result<()> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum_2(total, &ops, ops[0], 1)? {
            sum += total;
        }
    }
    println!("{sum}");
    Ok(())
}
