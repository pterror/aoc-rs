use std::collections::BTreeMap;
use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

pub struct Day9;

impl Solution for Day9 {
    type Input = Vec<u8>;

    fn day() -> u8 {
        9
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day9.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Vec<u8>> {
        input.iter().map(|x| x - b'0').collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
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
        let mut end = fs.len() - 1;
        let mut start = 0;
        while start < end {
            if fs[end] == None {
                end -= 1;
            } else if fs[start] != None {
                start += 1;
            } else {
                fs[start] = fs[end];
                fs[end] = None;
            }
        }
        fs.drain(end + 1..);
        let mut sum = 0;
        for (i, &x) in fs.iter().enumerate() {
            sum += i * x.unwrap_or(0);
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut files = Vec::<(usize, u8)>::new();
        let mut spaces = BTreeMap::<usize, u8>::new();
        let mut pos = 0;
        for (i, &size) in xs.iter().enumerate() {
            if i % 2 == 0 {
                files.push((pos, size));
            } else {
                spaces.insert(pos, size);
            }
            pos += size as usize;
        }
        for file_id in (0..files.len()).rev() {
            let (file_pos, file_size) = files[file_id];
            let mut queued_remove = None;
            for (&space_pos, &space_size) in spaces.iter() {
                if space_pos >= file_pos {
                    break;
                }
                if space_size >= file_size {
                    files[file_id].0 = space_pos;
                    queued_remove = Some((
                        space_pos,
                        space_pos + file_size as usize,
                        space_size - file_size,
                    ));
                    break;
                }
            }
            if let Some((old_space_pos, space_pos, space_size)) = queued_remove {
                spaces.remove(&old_space_pos);
                if space_size != 0 {
                    spaces.insert(space_pos, space_size);
                }
            }
        }
        let mut sum = 0;
        for (id, &(i, length)) in files.iter().enumerate() {
            sum += id * length as usize * (i + i + length as usize - 1) / 2;
        }
        Ok(sum)
    }
}
