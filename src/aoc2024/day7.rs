use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<String>> {
    let lines = read_lines("inputs/day7.txt")?;
    Ok(lines)
}

pub fn p1() -> Result<()> {
    let xs = parse()?;
    println!("{xs:?}");
    Ok(())
}

pub fn p2() -> Result<()> {
    let xs = parse()?;
    println!("{xs:?}");
    Ok(())
}
