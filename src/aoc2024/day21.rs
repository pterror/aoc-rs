use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::LazyLock;

use anyhow::Result;

use crate::util::*;

static NUM_PAD: LazyLock<HashMap<u8, (u8, u8)>> = LazyLock::new(|| {
    HashMap::from([
        (b'7', (0, 0)),
        (b'8', (0, 1)),
        (b'9', (0, 2)),
        (b'4', (1, 0)),
        (b'5', (1, 1)),
        (b'6', (1, 2)),
        (b'1', (2, 0)),
        (b'2', (2, 1)),
        (b'3', (2, 2)),
        (b'0', (3, 1)),
        (b'A', (3, 2)),
    ])
});

static DIR_PAD: LazyLock<HashMap<u8, (u8, u8)>> = LazyLock::new(|| {
    HashMap::from([
        (b'^', (0, 1)),
        (b'A', (0, 2)),
        (b'<', (1, 0)),
        (b'v', (1, 1)),
        (b'>', (1, 2)),
    ])
});

pub struct Day21;

impl Solution for Day21 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        21
    }

    fn default_input() -> Result<Vec<u8>> {
        // read_bytes!("inputs/aoc2024/day21.txt")
        // assert_eq!(num(b'A', b"", 2, &mut HashMap::new()), 2);
        if false {
            br"3A".to_vec().ok()
        } else {
            br"029A
980A
179A
456A
379A"
                .to_vec()
                .ok()
        }
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        let mut cache = HashMap::new();
        for x in xs {
            let (n, _) = usize::try_search(&x);
            let n = n.unwrap_or_default();
            let len = num(b'A', &x, 2, &mut cache);
            println!("{n} {len}");
            sum += n * len.len();
        }
        let mut entries = cache
            .iter()
            .map(|((a, b, c), v)| (c, *a as char, *b as char, v))
            .collect_vec();
        entries.sort();
        for (a, b, c, d) in entries {
            println!("{a} {b} {c} {d}");
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        let mut cache = HashMap::new();
        for x in xs {
            let (n, _) = usize::try_search(&x);
            let n = n.unwrap_or_default();
            let len = num(b'A', &x, 24, &mut cache);
            sum += n * len.len();
        }
        Ok(sum)
    }
}

fn num(a: u8, bs: &[u8], steps: u8, cache: &mut HashMap<(u8, u8, u8), String>) -> String {
    match bs {
        [b, bs @ ..] => format!(
            "{}{}",
            num_step(a, *b, steps, cache),
            num(*b, bs, steps, cache)
        ),
        _ => String::from(""),
    }
}

fn legal_num_step(dir: u8, i: u8, j: u8, i2: u8, j2: u8) -> bool {
    if i == 3 {
        dir != b'<' || j2 != 0
    } else if j == 0 {
        dir != b'v' || i2 != 3
    } else {
        true
    }
}

fn num_step(a: u8, b: u8, steps: u8, cache: &mut HashMap<(u8, u8, u8), String>) -> String {
    let &(i, j) = NUM_PAD.get(&a).unwrap();
    let &(i2, j2) = NUM_PAD.get(&b).unwrap();
    let vec1 = (if i > i2 { b'^' } else { b'v' }, i.abs_diff(i2));
    let vec2 = (if j > j2 { b'<' } else { b'>' }, j.abs_diff(j2));
    let len = vec![(vec1, vec2), (vec2, vec1)]
        .into_iter()
        .filter(|&((dir1, len1), _)| legal_num_step(dir1, i, j, i2, j2) && len1 != 0)
        .map(|((dir1, len1), (dir2, len2))| {
            let new_s = (0..len1)
                .map(|_| dir1)
                .chain((0..len2).map(|_| dir2))
                .chain(Some(b'A'))
                .collect_vec();
            let ret = dir(b'A', &new_s, steps, cache);
            // println!(" ### {ret} {}", String::from_utf8_lossy(&new_s));
            // println!(
            //     " ### {ret} {} {} {len1} {} {len2} {i},{j} {i2},{j2}",
            //     String::from_utf8_lossy(&new_s),
            //     dir1 as char,
            //     dir2 as char
            // );
            ret
        })
        .min_by(|a, b| usize::cmp(&a.len(), &b.len()))
        .unwrap();
    // println!("{} {} {len}", a as char, b as char);
    len
}

fn dir(a: u8, bs: &[u8], steps: u8, cache: &mut HashMap<(u8, u8, u8), String>) -> String {
    match bs {
        [b, bs @ ..] => format!(
            "{}{}",
            dir_step(a, *b, steps, cache),
            dir(*b, bs, steps, cache)
        ),
        _ => String::from(""),
    }
}

fn legal_dir_step(dir: u8, i: u8, j: u8, i2: u8, j2: u8) -> bool {
    if i == 0 {
        dir != b'<' || j2 != 0
    } else if j == 0 {
        dir != b'^' || i2 != 0
    } else {
        true
    }
}

fn dir_step(a: u8, b: u8, steps: u8, cache: &mut HashMap<(u8, u8, u8), String>) -> String {
    if steps == 0 {
        return String::from_utf8_lossy(&[a]).to_string();
        // let &(i, j) = DIR_PAD.get(&a).unwrap();
        // let &(i2, j2) = DIR_PAD.get(&b).unwrap();
        // return (i.abs_diff(i2) + j.abs_diff(j2) + 1) as _;
    } else if let Some(len) = cache.get(&(a, b, steps)) {
        return len.clone();
    }
    let &(i, j) = DIR_PAD.get(&a).unwrap();
    let &(i2, j2) = DIR_PAD.get(&b).unwrap();
    let vec1 = (if i > i2 { b'^' } else { b'v' }, i.abs_diff(i2));
    let vec2 = (if j > j2 { b'<' } else { b'>' }, j.abs_diff(j2));
    let len = vec![(vec1, vec2), (vec2, vec1)]
        .into_iter()
        .filter(|&((dir1, len1), (_, len2))| {
            let ret = legal_dir_step(dir1, i, j, i2, j2) && (len1 != 0 || len2 == 0);
            // println!(
            //     "{} {} {}   {} {} {len1} {len2} {i},{j} {i2},{j2}",
            //     legal_dir_step(dir1, i, j, i2, j2),
            //     len1 != 0,
            //     len2 == 0,
            //     dir1 as char,
            //     dir2 as char,
            // );
            ret
        })
        .map(|((dir1, len1), (dir2, len2))| {
            let new_s = (0..len1)
                .map(|_| dir1)
                .chain((0..len2).map(|_| dir2))
                .chain(Some(b'A'))
                .collect_vec();
            let ret = dir(b'A', &new_s, steps - 1, cache);
            // println!("{steps}>>> {ret} {}", String::from_utf8_lossy(&new_s));
            // println!(
            //     "{steps}>>> {ret} {} {} {len1} {} {len2} {i},{j} {i2},{j2}",
            //     String::from_utf8_lossy(&new_s),
            //     dir1 as char,
            //     dir2 as char
            // );
            ret
        })
        .min_by(|a, b| usize::cmp(&a.len(), &b.len()))
        .unwrap();
    // if steps == 1 {
    //     println!("ab {len} {}", i.abs_diff(i2) + j.abs_diff(j2) + 1);
    // }
    cache.insert((a, b, steps), len.clone());
    len
}
