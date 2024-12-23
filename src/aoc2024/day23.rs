use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use anyhow::Result;

use crate::util::*;

pub struct Day23;

impl Solution for Day23 {
    type Input = Vec<(Vec<u8>, Vec<u8>)>;

    fn day() -> u8 {
        23
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day23.txt")
        //         br"kh-tc
        // qp-kh
        // de-cg
        // ka-co
        // yn-aq
        // qp-ub
        // cg-tb
        // vc-aq
        // tb-ka
        // wh-tc
        // yn-cg
        // kh-ub
        // ta-co
        // de-co
        // tc-td
        // tb-wq
        // wh-td
        // ta-ka
        // td-qp
        // aq-cg
        // wq-ub
        // ub-vc
        // de-ta
        // wq-aq
        // wq-vc
        // wh-yn
        // ka-de
        // kh-ta
        // co-tc
        // wh-qp
        // tb-vc
        // td-yn"
        //             .to_vec()
        //             .ok()
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|x| {
                let what = x.split(|&c| c == b'-').collect_vec();
                (what[0].to_vec(), what[1].to_vec())
            })
            .collect_vec()
            .ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut parts = HashMap::<Vec<u8>, HashSet<Vec<u8>>>::new();
        for (a, b) in xs.iter() {
            parts.entry(a.clone()).or_default().insert(b.clone());
            parts.entry(b.clone()).or_default().insert(a.clone());
        }
        let mut sets = HashSet::new();
        for (a, b) in xs.iter() {
            let cs = parts.get(a);
            if let Some(cs) = cs {
                for c in cs {
                    if c == a || c == b {
                        continue;
                    }
                    let bp = parts.get(b);
                    if let Some(bp) = bp {
                        if !bp.contains(c) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let mut triple = vec![a, b, c];
                    triple.sort();
                    sets.insert((triple[0], triple[1], triple[2]));
                }
            }
        }
        let mut count = 0;
        for set in sets.iter() {
            if set.0[0] == b't' || set.1[0] == b't' || set.2[0] == b't' {
                count += 1;
                continue;
            }
        }
        Ok(count)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut parts = HashMap::<Vec<u8>, HashSet<Vec<u8>>>::new();
        for (a, b) in xs.iter() {
            parts.entry(a.clone()).or_default().insert(b.clone());
            parts.entry(b.clone()).or_default().insert(a.clone());
        }
        let mut sets = Vec::new();
        let mut computers = HashSet::new();
        for (a, b) in xs.iter() {
            computers.insert(a);
            computers.insert(b);
        }
        for &c in computers.iter() {
            sets.push(HashSet::from([c]));
        }
        for &c in computers.iter() {
            'outer: for set in sets.iter_mut() {
                if set.contains(c) {
                    continue;
                }
                let conn = parts.get(c);
                if let Some(conn) = conn {
                    for &nb in set.iter() {
                        if !conn.contains(nb) {
                            continue 'outer;
                        }
                    }
                    set.insert(c);
                }
            }
        }
        // println!("{:?}", sets.iter().map());
        sets.iter()
            .max_by(|&x, &y| x.len().cmp(&y.len()))
            .map(|x| {
                let mut vec = x.iter().map(|x| String::from_utf8_lossy(x)).collect_vec();
                vec.sort();
                vec.join(",")
            })
            .to_result()
    }
}
