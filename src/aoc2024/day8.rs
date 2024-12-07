use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<String>> {
    let lines = read_lines!("inputs/aoc2024/day8.txt")?;
    Ok(lines)
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    Ok(format!("{xs:?}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    Ok(format!("{xs:?}"))
}
