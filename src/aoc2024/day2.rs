use anyhow::Result;

use crate::util::{read_lines, to, CollectResult};

fn parse() -> Result<Vec<Vec<isize>>> {
    let lines = read_lines("inputs/day2.txt")?;
    let result = lines
        .iter()
        .map(|line| line.split(" ").map(to::<isize>).collect_result());
    Ok(result.collect_result()?)
}

pub fn p1() -> Result<()> {
    let xs = parse()?;
    let mut count = 0;
    for x in xs {
        let diffs = x
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, n)| n - x[i])
            .collect::<Vec<_>>();
        if !diffs.iter().all(|n| *n < 0) && !diffs.iter().all(|n| *n > 0) {
            continue;
        }
        if !diffs.iter().all(|n| n.abs() >= 1 && n.abs() <= 3) {
            continue;
        }
        count += 1;
    }
    println!("{count}");
    Ok(())
}

pub fn p2() -> Result<()> {
    let xs = parse()?;
    let mut count = 0;
    'outer: for x in xs {
        for (i, _) in x.iter().enumerate() {
            let x = x
                .iter()
                .take(i)
                .chain(x.iter().skip(i + 1))
                .collect::<Vec<_>>();
            let diffs = x
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, n)| *n - x[i])
                .collect::<Vec<_>>();
            if !diffs.iter().all(|n| *n < 0) && !diffs.iter().all(|n| *n > 0) {
                continue;
            }
            if !diffs.iter().all(|n| n.abs() >= 1 && n.abs() <= 3) {
                continue;
            }
            count += 1;
            continue 'outer;
        }
    }
    println!("{count}");
    Ok(())
}
