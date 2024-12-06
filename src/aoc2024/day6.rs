use std::collections::HashSet;

use anyhow::Result;

use crate::util::read_lines;

fn parse() -> Result<Vec<Vec<char>>> {
    let lines = read_lines!("inputs/aoc2024/day6.txt")?;
    Ok(lines.iter().map(|x| x.chars().collect()).collect())
}

pub fn p1() -> Result<()> {
    let mut xs = parse()?;
    let mut i = 0;
    let mut j = 0;
    let mut dir = (-1, 0);
    'outer: for (i_, l) in xs.iter().enumerate() {
        for (j_, c) in l.iter().enumerate() {
            if *c == '^' {
                i = i_ as isize;
                j = j_ as isize;
                break 'outer;
            }
        }
    }
    let mut count = 0;
    loop {
        let c = xs[i as usize][j as usize];
        if c == '#' {
            i -= dir.0;
            j -= dir.1;
            dir = (dir.1, -dir.0);
        } else if c != 'X' {
            count += 1;
            xs[i as usize][j as usize] = 'X';
        }
        i += dir.0;
        j += dir.1;
        if i < 0 || j < 0 || (i as usize) >= xs.len() || (j as usize) >= xs[0].len() {
            break;
        }
    }
    println!("{count}");
    Ok(())
}

fn is_loop(xs: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let mut i = i;
    let mut j = j;
    let mut dir = (-1, 0);
    let mut seen = HashSet::new();
    loop {
        let c = xs[i as usize][j as usize];
        if c == '#' {
            if seen.contains(&(i, j, dir)) {
                return true;
            }
            seen.insert((i, j, dir));
            i -= dir.0;
            j -= dir.1;
            dir = (dir.1, -dir.0);
        }
        i += dir.0;
        j += dir.1;
        if i < 0 || j < 0 || (i as usize) >= xs.len() || (j as usize) >= xs[0].len() {
            return false;
        }
    }
}

pub fn p2() -> Result<()> {
    let mut xs = parse()?;
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    'outer: for (i_, l) in xs.iter().enumerate() {
        for (j_, c) in l.iter().enumerate() {
            if *c == '^' {
                i = i_ as i32;
                j = j_ as i32;
                break 'outer;
            }
        }
    }
    let mut i2 = i;
    let mut j2 = j;
    let mut dir = (-1, 0);
    loop {
        let c = xs[i2 as usize][j2 as usize];
        if c == '#' {
            i2 -= dir.0;
            j2 -= dir.1;
            dir = (dir.1, -dir.0);
        } else if c != 'X' {
            xs[i2 as usize][j2 as usize] = '#';
            if is_loop(&xs, i, j) {
                count += 1;
            }
            xs[i2 as usize][j2 as usize] = 'X';
        }
        i2 += dir.0;
        j2 += dir.1;
        if i2 < 0 || j2 < 0 || (i2 as usize) >= xs.len() || (j2 as usize) >= xs[0].len() {
            break;
        }
    }
    println!("{count}");
    Ok(())
}
