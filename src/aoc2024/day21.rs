use std::{collections::HashMap, fmt::Debug, sync::LazyLock};

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
        br"029A
980A
179A
456A
379A"
            .to_vec()
            .ok()
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut sum = 0;
        // for x in xs {
        //     let (n, _) = usize::try_search(&x);
        //     let n = n.unwrap_or_default();
        //     let mut code2 = Vec::new();
        //     let mut c = b'A';
        //     for d in x {
        //         code2.extend(nummov.get(&(c, d)).to_result()?);
        //         c = d;
        //     }
        //     let mut code1 = Vec::new();
        //     let mut c = b'A';
        //     for &d in code2.iter() {
        //         code1.extend(dirmov.get(&(c, d)).to_result()?);
        //         c = d;
        //     }
        //     let mut code0 = Vec::new();
        //     let mut c = b'A';
        //     for &d in code1.iter() {
        //         code0.extend(dirmov.get(&(c, d)).to_result()?);
        //         c = d;
        //     }
        //     println!("{n}");
        //     println!("{}", String::from_utf8_lossy(&code2).to_owned());
        //     println!("{}", String::from_utf8_lossy(&code1).to_owned());
        //     println!("{}", String::from_utf8_lossy(&code0).to_owned());
        //     println!("{n} {}", code0.len());
        //     sum += n * code0.len();
        // }
        Ok(0)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }
}

fn num_step(a: u8, b: u8, steps: u8, cache: &mut HashMap<(u8, u8, u8, u8, u8), usize>) -> usize {
    let &(i, j) = NUM_PAD.get(&a).unwrap_or(&(0, 0));
    let &(i2, j2) = NUM_PAD.get(&b).unwrap_or(&(0, 0));
    // if i == 0 {
    //     //
    // }
    0
}

fn dir_step(a: u8, b: u8, steps: u8, cache: &mut HashMap<(u8, u8, u8, u8, u8), usize>) -> usize {
    let &(i, j) = DIR_PAD.get(&a).unwrap_or(&(0, 0));
    let &(i2, j2) = DIR_PAD.get(&b).unwrap_or(&(0, 0));
    // if let Some(&steps) = cache.get(&(i, j, i2, j2, steps)) {
    //     steps
    // } else {
    //     let mut len = 0;
    //     len
    // }
    0
}
