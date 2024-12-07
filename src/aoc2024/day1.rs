use anyhow::Result;

use crate::util::{read_lines, CollectVec};

fn parse() -> Result<(Vec<usize>, Vec<usize>)> {
    let lines = read_lines!("inputs/aoc2024/day1.txt")?;
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();
    for line in lines {
        let nums = line.split("   ").collect_vec();
        a.push(nums[0].parse()?);
        b.push(nums[1].parse()?);
    }
    return Ok((a, b));
}

pub fn p1() -> Result<String> {
    let (mut a, mut b) = parse()?;
    a.sort();
    b.sort();
    let mut sum = 0usize;
    for (a, b) in a.iter().zip(b) {
        sum += a.abs_diff(b);
    }
    Ok(format!("{sum}"))
}

pub fn p2() -> Result<String> {
    let (mut a, b) = parse()?;
    a.sort();
    let mut sum = 0usize;
    for a in a.iter() {
        sum += a * b.iter().filter(|b| a == *b).count();
    }
    Ok(format!("{sum}"))
}
