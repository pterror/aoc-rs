use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<(Vec<usize>, Vec<usize>)> {
    let lines = read_lines("inputs/day1.txt")?;
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();
    for line in lines {
        let nums = line.split("   ").collect::<Vec<_>>();
        a.push(nums[0].parse()?);
        b.push(nums[1].parse()?);
    }
    return Ok((a, b));
}

pub fn p1() -> Result<()> {
    let (mut a, mut b) = parse()?;
    a.sort();
    b.sort();
    let mut sum = 0usize;
    for (a, b) in a.iter().zip(b) {
        sum += a.abs_diff(b);
    }
    println!("{sum}");
    Ok(())
}

pub fn p2() -> Result<()> {
    let (mut a, b) = parse()?;
    a.sort();
    let mut sum = 0usize;
    for a in a.iter() {
        sum += a * b.iter().filter(|b| a == *b).count();
    }
    println!("{sum}");
    Ok(())
}
