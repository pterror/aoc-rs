use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem::take,
};

use anyhow::{Error, Result};

use crate::util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    AND,
    OR,
    XOR,
}

pub struct Day24;

impl Solution for Day24 {
    type Input = (HashMap<Vec<u8>, bool>, Vec<(Vec<u8>, Op, Vec<u8>, Vec<u8>)>);

    fn day() -> u8 {
        24
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day24.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let parts = lines.split(|l| l.len() == 0).collect_vec();
        let xys = parts[0];
        let mut map = HashMap::new();
        for line in xys {
            map.insert(line[..3].to_vec(), line[5] == b'1');
        }
        let eqns = parts[1];
        let ops = HashMap::from([
            (b"AND".to_vec(), Op::AND),
            (b"OR".to_vec(), Op::OR),
            (b"XOR".to_vec(), Op::XOR),
        ]);
        let mut parsed = Vec::new();
        for eqn in eqns {
            let arg1 = eqn[..3].to_vec();
            let l = eqn.len();
            let op = *ops.get(&eqn[4..l - 11].to_vec()).unwrap_or(&Op::AND);
            let arg2 = eqn[l - 10..l - 7].to_vec();
            let res = eqn[l - 3..].to_vec();
            parsed.push((arg1, op, arg2, res));
        }
        Ok((map, parsed))
    }

    fn p1((regs, eqns): Self::Input) -> Result<impl Debug> {
        let mut regs = regs.clone();
        let mut eqns = eqns.iter().collect::<HashSet<_>>();
        while eqns.len() != 0 {
            let mut to_remove = None;
            for item @ (a, op, b, r) in eqns.iter() {
                if let (Some(a), Some(b)) = (regs.get(a), regs.get(b)) {
                    let val = match op {
                        Op::AND => a & b,
                        Op::OR => a | b,
                        Op::XOR => a ^ b,
                    };
                    regs.insert(r.clone(), val);
                    to_remove = Some((*item).clone());
                    break;
                }
            }
            if let Some(to_remove) = to_remove {
                eqns.remove(&to_remove);
            }
        }
        let mut z = 0usize;
        for (reg, bit) in regs {
            if reg[0] != b'z' {
                continue;
            }
            let shift = reg[1..].parse::<usize>();
            z |= (if bit { 1 } else { 0 }) << shift;
        }
        Ok(z)
    }

    fn p2((regs, eqns): Self::Input) -> Result<impl Debug> {
        let mut renames = HashMap::new();
        let mut renamed = HashSet::new();
        for (reg, _) in regs.iter() {
            renames.insert(reg.clone(), reg.clone());
        }
        let mut swaps = HashMap::from([
            // (b"qjj".to_vec(), b"qjj".to_vec()),
            (b"gjc".to_vec(), b"qjj".to_vec()),
            (b"qjj".to_vec(), b"gjc".to_vec()),
            // d11 (qjj) -> t11 (gjc)
            // qsb OR e39 -> c40
            // qsb OR vsm -> chr
        ]);
        let mut eqns = eqns.iter().collect::<HashSet<_>>();
        while eqns.len() != 0 {
            let mut to_remove = None;
            for item @ (a, op, b, r) in eqns.iter() {
                if let (Some(a), Some(b)) = (renames.get(a).cloned(), renames.get(b).cloned()) {
                    let res_c = match (a[0], op, b[0]) {
                        (b'x', Op::AND, b'y') | (b'y', Op::AND, b'x') => b'd',
                        (b'd', Op::OR, b'e') | (b'e', Op::OR, b'd') => b'c',
                        (b'x', Op::XOR, b'y') | (b'y', Op::XOR, b'x') => {
                            if a[1] == b'0' && a[2] == b'0' {
                                b'z'
                            } else {
                                b't'
                            }
                        }
                        (b't', Op::XOR, b'c') | (b'c', Op::XOR, b't') => b'z',
                        (b'c', Op::AND, b't') | (b't', Op::AND, b'c') => b'e',
                        _ => {
                            continue;
                        }
                    };
                    if res_c == b'z' && r[0] != b'z' {
                        let res_s = vec![b'z', a[1], a[2]];
                        swaps.insert(res_s.clone(), r.clone());
                        swaps.insert(r.clone(), res_s.clone());
                    }
                    let res = if res_c != b'c' {
                        vec![res_c, a[1], a[2]]
                    } else {
                        let carry = if a[2] == b'9' { 1 } else { 0 };
                        let last = if a[2] == b'9' { b'0' } else { a[2] + 1 };
                        vec![res_c, a[1] + carry, last]
                    };
                    renames.insert(swaps.get(r).unwrap_or(&r).clone(), res.clone());
                    renamed.insert((a, op, b, res.clone()));
                    to_remove = Some((*item).clone());
                    break;
                } else {
                    let a = renames.get(a).unwrap_or(a).clone();
                    let b = renames.get(b).unwrap_or(b).clone();
                    match (a[0], op, b[0]) {
                        (b'd', Op::OR, _) => {
                            renames.insert(b, vec![b'e', a[1], a[2]]);
                        }
                        (_, Op::OR, b'd') => {
                            renames.insert(a, vec![b'e', b[1], b[2]]);
                        }
                        (_, Op::XOR, b't') => {
                            renames.insert(a, vec![b'z', b[1], b[2]]);
                        }
                        (b't', Op::XOR, _) => {
                            renames.insert(b, vec![b'z', a[1], a[2]]);
                        }
                        (_, Op::AND, b't') => {
                            renames.insert(a, vec![b'c', b[1], b[2]]);
                        }
                        (b't', Op::AND, _) => {
                            renames.insert(b, vec![b'c', a[1], a[2]]);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(to_remove) = to_remove {
                eqns.remove(&to_remove);
            } else {
                // println!(
                //     "{:?} {}",
                //     renames
                //         .iter()
                //         .map(|(a, b)| format!(
                //             "{} {}",
                //             String::from_utf8_lossy(&a),
                //             String::from_utf8_lossy(&b),
                //         ))
                //         .collect_vec(),
                //     renames.len()
                // );
                let mut what = HashSet::new();
                for (a, _, b, r) in eqns.iter() {
                    if !renames.contains_key(a) {
                        what.insert(a);
                    }
                    if !renames.contains_key(b) {
                        what.insert(b);
                    }
                    if !renames.contains_key(r) {
                        what.insert(r);
                    }
                }
                // println!(
                //     "{:?}",
                //     String::from_utf8_lossy(
                //         renames
                //             .iter()
                //             .filter(|(_, r)| r == &&b"t11".to_vec())
                //             .take(1)
                //             .collect_vec()[0]
                //             .0
                //     )
                // );
                println!(
                    "{:?}",
                    what.iter()
                        .map(|s| String::from_utf8_lossy(s))
                        .collect_vec()
                );
                println!("{}", eqns.len());
                println!(
                    "{}\n",
                    eqns.iter()
                        .map(|(a, op, b, r)| format!(
                            "{} {op:?} {} -> {}",
                            String::from_utf8_lossy(renames.get(a).unwrap_or(a)),
                            String::from_utf8_lossy(renames.get(b).unwrap_or(b)),
                            String::from_utf8_lossy(renames.get(r).unwrap_or(r)),
                        ))
                        .collect_vec()
                        .join("\n")
                );
                println!(
                    "{}",
                    eqns.iter()
                        .map(|(a, op, b, r)| format!(
                            "{} {op:?} {} -> {}",
                            String::from_utf8_lossy(a),
                            String::from_utf8_lossy(b),
                            String::from_utf8_lossy(r),
                        ))
                        .collect_vec()
                        .join("\n")
                );
                return Err(Error::msg("day 24 part 2: mistakes found!"));
            }
        }
        // x## & y## => d##
        // d## | e## => c##+1
        // x## ^ y## => t##
        // t## ^ c## => z##
        // t## & c## => e##
        println!(
            "{:?}",
            swaps
                .iter()
                .map(|(a, b)| format!(
                    "{} {}",
                    String::from_utf8_lossy(a),
                    String::from_utf8_lossy(b)
                ))
                .collect_vec()
        );
        let mut mistake_strs = swaps
            .iter()
            .flat_map(|(k, v)| vec![String::from_utf8_lossy(k), String::from_utf8_lossy(v)])
            .collect::<HashSet<_>>()
            .iter()
            .map(|x| x.clone())
            .collect_vec();
        mistake_strs.sort();
        Ok(mistake_strs.join(","))
    }
}
