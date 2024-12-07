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

fn any_sum(a: usize, ops: &[usize]) -> bool {
    match ops {
        [] => a == 0,
        [n] => a == *n,
        [rest @ .., n] => (a >= *n && any_sum(a - n, rest)) || (a % n == 0 && any_sum(a / n, rest)),
    }
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum(total, &ops) {
            sum += total;
        }
    }
    Ok(format!("{sum}"))
}

fn any_sum_2(a: usize, ops: &[usize]) -> bool {
    match ops {
        [] => a == 0,
        [n] => a == *n,
        [rest @ .., n] => {
            (a >= *n && any_sum_2(a - n, rest)) || (a % n == 0 && any_sum_2(a / n, rest)) || {
                let modulo = 10usize.pow(n.ilog10() + 1);
                a % modulo == *n && any_sum_2(a / modulo, rest)
            }
        }
    }
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut sum = 0;
    for (total, ops) in xs {
        if any_sum_2(total, &ops) {
            sum += total;
        }
    }
    Ok(format!("{sum}"))
}
