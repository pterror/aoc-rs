use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

type File = (u8, usize);

#[derive(Debug)]
enum FsEntry {
    File(File),
    Space(u8, Vec<File>),
}

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
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut fs = Vec::new();
        for (i, &x) in xs.iter().enumerate() {
            if i % 2 == 0 {
                let id = i / 2;
                fs.push(FsEntry::File((x, id)))
            } else {
                fs.push(FsEntry::Space(x, Vec::with_capacity(0)))
            }
        }
        let mut end = fs.len();
        let mut start = 0;
        while start < end {
            end -= 1;
            while let FsEntry::File(_) = fs[start] {
                start += 1;
            }
            while let FsEntry::Space(_, _) = fs[end] {
                end -= 1;
            }
            if let FsEntry::File(file @ (f_len, _)) = fs[end] {
                let mut curr = start;
                while curr < end {
                    if let FsEntry::Space(space_left, files) = &mut fs[curr] {
                        if *space_left >= f_len {
                            *space_left -= f_len;
                            files.push(file);
                            if *space_left == 0 && curr == start {
                                start += 1;
                            }
                            fs[end] = FsEntry::Space(f_len, Vec::new());
                            break;
                        }
                    }
                    curr += 1;
                }
            }
        }
        let mut sum = 0;
        let mut i = 0;
        for entry in fs.iter() {
            match entry {
                FsEntry::File((length, id)) => {
                    let length = *length as usize;
                    sum += *id * length * (i + i + length - 1) / 2;
                    i += length;
                }
                FsEntry::Space(free_space, files) => {
                    for (length, id) in files {
                        let length = *length as usize;
                        sum += *id * length * (i + i + length - 1) / 2;
                        i += length;
                    }
                    i += *free_space as usize;
                }
            }
        }
        Ok(sum)
    }
}
