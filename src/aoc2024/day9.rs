use anyhow::Result;

use crate::util::{read_string, CollectVec};

fn parse() -> Result<Vec<u8>> {
    Ok(read_string!("inputs/aoc2024/day9.txt")?
        .chars()
        .map(|x| (x as u8) - b'0')
        .collect_vec())
}

pub fn p1() -> Result<String> {
    let xs = parse()?;
    let mut parts = Vec::<Vec<Option<usize>>>::new();
    for (i, &x) in xs.iter().enumerate() {
        if i % 2 == 0 {
            let id = i / 2;
            parts.push((0..x).map(|_| Some(id)).collect_vec())
        } else {
            parts.push((0..x).map(|_| None).collect_vec())
        }
    }
    let mut fs = parts.iter().flatten().collect_vec();
    let mut end = fs.len() - 1;
    let mut start = 0;
    while start < end {
        if *fs[end] == None {
            end -= 1;
        } else if *fs[start] != None {
            start += 1;
        } else {
            fs[start] = fs[end];
            fs[end] = &None;
        }
    }
    fs.drain(end + 1..);
    let mut sum = 0;
    for (i, &&x) in fs.iter().enumerate() {
        sum += i * x.unwrap_or(0);
    }
    Ok(format!("{sum}"))
}

pub fn p2() -> Result<String> {
    let xs = parse()?;
    let mut parts = Vec::<Vec<Option<usize>>>::new();
    for (i, &x) in xs.iter().enumerate() {
        if i % 2 == 0 {
            let id = i / 2;
            parts.push((0..x).map(|_| Some(id)).collect_vec())
        } else {
            parts.push((0..x).map(|_| None).collect_vec())
        }
    }
    let mut fs = parts.iter().flatten().map(|x| *x).collect_vec();
    let mut end = fs.len();
    let mut start = 0;
    while start < end {
        end -= 1;
        while fs[start] != None {
            start += 1;
        }
        while fs[end] == None {
            end -= 1;
        }
        let num = fs[end];
        let mut num_size = 1;
        while end > 0 && fs[end - 1] == num {
            end -= 1;
            num_size += 1;
        }
        let mut sp_start = start;
        while sp_start < end {
            while sp_start < end && fs[sp_start] != None {
                sp_start += 1;
            }
            let mut sp_size = 1;
            let mut sp_curr = sp_start;
            while fs[sp_curr + 1] == None {
                sp_curr += 1;
                sp_size += 1;
            }
            if sp_size >= num_size {
                for i in 0..num_size {
                    fs[sp_start + i] = num;
                    fs[end + i] = None;
                }
                break;
            } else {
                sp_start += 1;
            }
        }
    }
    let mut sum = 0;
    for (i, &x) in fs.iter().enumerate() {
        sum += i * x.unwrap_or(0);
    }
    Ok(format!("{sum}"))
}
