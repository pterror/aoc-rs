use anyhow::Result;
use regex::Regex;

use crate::util::read_file;

fn parse() -> Result<String> {
    let str = read_file!("inputs/aoc2024/day3.txt")?;
    Ok(str)
}

pub fn p1() -> Result<()> {
    let xs = parse()?;
    let pat = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut total = 0isize;
    for instr in pat.captures_iter(xs.as_str()) {
        total += instr[1].parse::<isize>()? * instr[2].parse::<isize>()?;
    }
    println!("{total}");
    Ok(())
}

pub fn p2() -> Result<()> {
    let xs = parse()?;
    let pat = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;
    let mut enabled = true;
    let mut total = 0isize;
    for instr in pat.captures_iter(xs.as_str()) {
        if instr[0] == *"do()" {
            enabled = true;
        } else if instr[0] == *"don't()" {
            enabled = false;
        } else if enabled {
            total += instr[1].parse::<isize>()? * instr[2].parse::<isize>()?;
        }
    }
    println!("{total}");
    Ok(())
}
