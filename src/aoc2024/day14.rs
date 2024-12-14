use std::fmt::Debug;

use anyhow::{Error, Result};

use crate::util::*;

pub struct Day14;

impl Solution for Day14 {
    type Input = Vec<(isize, isize, isize, isize)>;

    fn day() -> u8 {
        14
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day14.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input
            .lines()
            .map(|line| {
                let (a, i) = isize::try_search(&line);
                let (b, i) = isize::try_search_with_start(&line, i);
                let (c, i) = isize::try_search_with_start(&line, i);
                let (d, _) = isize::try_search_with_start(&line, i);
                if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
                    Ok((a, b, c, d))
                } else {
                    Err(Error::msg(format!(
                        "day 14: parse error: {a:?} {b:?} {c:?} {d:?}"
                    )))
                }
            })
            .collect_result()
    }

    fn p1(mut xs: Self::Input) -> Result<impl Debug> {
        let w = 101;
        let h = 103;
        let wh = w / 2;
        let hh = h / 2;
        for _ in 0..100 {
            for (x, y, dx, dy) in xs.iter_mut() {
                *x = (*x + *dx + w) % w;
                *y = (*y + *dy + h) % h;
            }
        }
        let mut q0 = 0;
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        for &(x, y, _, _) in xs.iter() {
            if x < wh {
                if y < hh {
                    q0 += 1;
                } else if y > hh {
                    q1 += 1;
                }
            } else if x > wh {
                if y < hh {
                    q2 += 1;
                } else if y > hh {
                    q3 += 1;
                }
            }
        }
        Ok(q0 * q1 * q2 * q3)
    }

    fn p2(mut xs: Self::Input) -> Result<impl Debug> {
        let w = 101;
        let h = 103;
        let threshold = 2;
        let mut hi = 0;
        let mut hi_it = 0;
        for iter in 0..(w * h) {
            for (x, y, dx, dy) in xs.iter_mut() {
                *x = (*x + *dx + w) % w;
                *y = (*y + *dy + h) % h;
            }
            xs.sort();
            let mut count = 0;
            let mut last = (0, 0);
            for &(x, y, _, _) in xs.iter() {
                if x.abs_diff(last.0) < threshold && y.abs_diff(last.1) < threshold {
                    count += 1;
                }
                last = (x, y);
            }
            if count > hi {
                hi = count;
                hi_it = iter;
            }
        }
        Ok(hi_it + 1)
    }
}
