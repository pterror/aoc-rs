use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use crate::util::*;

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
        let nummov = HashMap::from([
            ((b'A', b'A'), b"A".to_vec()),
            ((b'0', b'0'), b"A".to_vec()),
            ((b'1', b'1'), b"A".to_vec()),
            ((b'2', b'2'), b"A".to_vec()),
            ((b'3', b'3'), b"A".to_vec()),
            ((b'4', b'4'), b"A".to_vec()),
            ((b'5', b'5'), b"A".to_vec()),
            ((b'6', b'6'), b"A".to_vec()),
            ((b'7', b'7'), b"A".to_vec()),
            ((b'8', b'8'), b"A".to_vec()),
            ((b'9', b'9'), b"A".to_vec()),
            ((b'A', b'0'), b"<A".to_vec()),
            ((b'A', b'1'), b"^<<A".to_vec()),
            ((b'A', b'2'), b"^<A".to_vec()),
            ((b'A', b'3'), b"^A".to_vec()),
            ((b'A', b'4'), b"^^<<A".to_vec()),
            ((b'A', b'5'), b"^^<A".to_vec()),
            ((b'A', b'6'), b"^^A".to_vec()),
            ((b'A', b'7'), b"^^^<<A".to_vec()),
            ((b'A', b'8'), b"^^^<A".to_vec()),
            ((b'A', b'9'), b"^^^A".to_vec()),
            ((b'0', b'A'), b">A".to_vec()),
            ((b'0', b'1'), b"^<A".to_vec()),
            ((b'0', b'2'), b"^A".to_vec()),
            ((b'0', b'3'), b"^>A".to_vec()),
            ((b'0', b'4'), b"^^<A".to_vec()),
            ((b'0', b'5'), b"^^A".to_vec()),
            ((b'0', b'6'), b"^^>A".to_vec()),
            ((b'0', b'7'), b"^^^<A".to_vec()),
            ((b'0', b'8'), b"^^^A".to_vec()),
            ((b'0', b'9'), b"^^^>A".to_vec()),
            ((b'1', b'A'), b">>vA".to_vec()),
            ((b'1', b'0'), b">vA".to_vec()),
            ((b'1', b'2'), b">A".to_vec()),
            ((b'1', b'3'), b">>A".to_vec()),
            ((b'1', b'4'), b"^A".to_vec()),
            ((b'1', b'5'), b"^>A".to_vec()),
            ((b'1', b'6'), b"^>>A".to_vec()),
            ((b'1', b'7'), b"^^A".to_vec()),
            ((b'1', b'8'), b"^^>A".to_vec()),
            ((b'1', b'9'), b"^^>>A".to_vec()),
            ((b'2', b'A'), b">vA".to_vec()),
            ((b'2', b'0'), b"vA".to_vec()),
            ((b'2', b'1'), b"<A".to_vec()),
            ((b'2', b'3'), b">A".to_vec()),
            ((b'2', b'4'), b"^<A".to_vec()),
            ((b'2', b'5'), b"^A".to_vec()),
            ((b'2', b'6'), b"^>A".to_vec()),
            ((b'2', b'7'), b"^^<A".to_vec()),
            ((b'2', b'8'), b"^^A".to_vec()),
            ((b'2', b'9'), b"^^>A".to_vec()),
            ((b'3', b'A'), b"vA".to_vec()),
            ((b'3', b'0'), b"v<A".to_vec()),
            ((b'3', b'1'), b"<<A".to_vec()),
            ((b'3', b'2'), b"<A".to_vec()),
            ((b'3', b'4'), b"^<<A".to_vec()),
            ((b'3', b'5'), b"^<A".to_vec()),
            ((b'3', b'6'), b"^A".to_vec()),
            ((b'3', b'7'), b"^^<<A".to_vec()),
            ((b'3', b'8'), b"^^<A".to_vec()),
            ((b'3', b'9'), b"^^A".to_vec()),
            ((b'4', b'A'), b">>vvA".to_vec()),
            ((b'4', b'0'), b">vvA".to_vec()),
            ((b'4', b'1'), b"vA".to_vec()),
            ((b'4', b'2'), b">vA".to_vec()),
            ((b'4', b'3'), b">>vA".to_vec()),
            ((b'4', b'5'), b">A".to_vec()),
            ((b'4', b'6'), b">>A".to_vec()),
            ((b'4', b'7'), b"^A".to_vec()),
            ((b'4', b'8'), b"^>A".to_vec()),
            ((b'5', b'9'), b"^>>A".to_vec()),
            ((b'5', b'A'), b">vvA".to_vec()),
            ((b'5', b'0'), b"vvA".to_vec()),
            ((b'5', b'1'), b"v<A".to_vec()),
            ((b'5', b'2'), b"vA".to_vec()),
            ((b'5', b'3'), b">vA".to_vec()),
            ((b'5', b'4'), b"<A".to_vec()),
            ((b'5', b'6'), b">A".to_vec()),
            ((b'5', b'7'), b"^<A".to_vec()),
            ((b'5', b'8'), b"^A".to_vec()),
            ((b'5', b'9'), b"^>A".to_vec()),
            ((b'6', b'A'), b"vvA".to_vec()),
            ((b'6', b'0'), b"vv<A".to_vec()),
            ((b'6', b'1'), b"v<<A".to_vec()),
            ((b'6', b'2'), b"v<A".to_vec()),
            ((b'6', b'3'), b"vA".to_vec()),
            ((b'6', b'4'), b"<<A".to_vec()),
            ((b'6', b'5'), b"<A".to_vec()),
            ((b'6', b'7'), b"^<<A".to_vec()),
            ((b'6', b'8'), b"^<A".to_vec()),
            ((b'6', b'9'), b"^A".to_vec()),
            ((b'7', b'A'), b">>vvvA".to_vec()),
            ((b'7', b'0'), b">vvvA".to_vec()),
            ((b'7', b'1'), b"vvA".to_vec()),
            ((b'7', b'2'), b">vvA".to_vec()),
            ((b'7', b'3'), b">>vvA".to_vec()),
            ((b'7', b'4'), b"vA".to_vec()),
            ((b'7', b'5'), b">vA".to_vec()),
            ((b'7', b'6'), b">>vA".to_vec()),
            ((b'7', b'8'), b">A".to_vec()),
            ((b'7', b'9'), b">>A".to_vec()),
            ((b'8', b'A'), b">vvvA".to_vec()),
            ((b'8', b'0'), b"vvvA".to_vec()),
            ((b'8', b'1'), b"vv<A".to_vec()),
            ((b'8', b'2'), b"vvA".to_vec()),
            ((b'8', b'3'), b">vvA".to_vec()),
            ((b'8', b'4'), b"v<A".to_vec()),
            ((b'8', b'5'), b"vA".to_vec()),
            ((b'8', b'6'), b">vA".to_vec()),
            ((b'8', b'7'), b"<A".to_vec()),
            ((b'8', b'9'), b">A".to_vec()),
            ((b'9', b'A'), b"vvvA".to_vec()),
            ((b'9', b'0'), b"vvv<A".to_vec()),
            ((b'9', b'1'), b"vv<<A".to_vec()),
            ((b'9', b'2'), b"vv<A".to_vec()),
            ((b'9', b'3'), b"vvA".to_vec()),
            ((b'9', b'4'), b"v<<A".to_vec()),
            ((b'9', b'5'), b"v<A".to_vec()),
            ((b'9', b'6'), b"vA".to_vec()),
            ((b'9', b'7'), b"<<A".to_vec()),
            ((b'9', b'8'), b"<A".to_vec()),
        ]);
        let dirmov = HashMap::from([
            ((b'A', b'A'), b"A".to_vec()),
            ((b'^', b'^'), b"A".to_vec()),
            ((b'<', b'<'), b"A".to_vec()),
            ((b'v', b'v'), b"A".to_vec()),
            ((b'>', b'>'), b"A".to_vec()),
            ((b'A', b'^'), b"<A".to_vec()),
            ((b'A', b'>'), b"vA".to_vec()),
            ((b'A', b'v'), b"v<A".to_vec()),
            ((b'A', b'<'), b"v<<A".to_vec()),
            ((b'^', b'A'), b">A".to_vec()),
            ((b'^', b'>'), b">vA".to_vec()),
            ((b'^', b'v'), b"vA".to_vec()),
            ((b'^', b'<'), b"v<A".to_vec()),
            ((b'<', b'A'), b">>^A".to_vec()),
            ((b'<', b'^'), b">^A".to_vec()),
            ((b'<', b'>'), b">>A".to_vec()),
            ((b'<', b'v'), b">A".to_vec()),
            ((b'>', b'A'), b"^A".to_vec()),
            ((b'>', b'^'), b"^<A".to_vec()),
            ((b'>', b'<'), b"<<A".to_vec()),
            ((b'>', b'v'), b"<A".to_vec()),
            ((b'v', b'A'), b">^A".to_vec()),
            ((b'v', b'^'), b"^A".to_vec()),
            ((b'v', b'<'), b"<A".to_vec()),
            ((b'v', b'>'), b">A".to_vec()),
        ]);
        for x in xs {
            let (n, _) = usize::try_search(&x);
            let n = n.unwrap_or_default();
            let mut code2 = Vec::new();
            let mut c = b'A';
            for d in x {
                code2.extend(nummov.get(&(c, d)).to_result()?);
                c = d;
            }
            let mut code1 = Vec::new();
            let mut c = b'A';
            for &d in code2.iter() {
                code1.extend(dirmov.get(&(c, d)).to_result()?);
                c = d;
            }
            let mut code0 = Vec::new();
            let mut c = b'A';
            for &d in code1.iter() {
                code0.extend(dirmov.get(&(c, d)).to_result()?);
                c = d;
            }
            println!("{n}");
            println!("{}", String::from_utf8_lossy(&code2).to_owned());
            println!("{}", String::from_utf8_lossy(&code1).to_owned());
            println!("{}", String::from_utf8_lossy(&code0).to_owned());
            println!("{n} {}", code0.len());
            sum += n * code0.len();
        }
        Ok(sum)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        Ok(xs)
    }
}

// fn step0(i: i32, j: i32, numpad: &HashMap<u8, (i32, i32)>, dirpad: &HashMap<u8, (i32, i32)>) {
//     //
// }

// fn step1(i: i32, j: i32, dirpad: &HashMap<u8, (i32, i32)>) {
//     //
// }

// fn step2(i: i32, j: i32, dirpad: &HashMap<u8, (i32, i32)>) {
//     //
// }
