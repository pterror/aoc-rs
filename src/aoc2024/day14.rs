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

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let w = 101;
        let h = 103;
        let wh = w / 2;
        let hh = h / 2;
        let steps = 100;
        let mut q0 = 0;
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let xs = xs.iter().map(|(x, y, dx, dy)| {
            (
                ((x + dx * steps) % w + w) % w,
                ((y + dy * steps) % h + h) % h,
            )
        });
        for (x, y) in xs {
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

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let w = 101;
        let h = 103;
        let mut hi = 0;
        let mut hi_it = 0;
        let mut hs = vec![vec![0; h as usize]; w as usize];
        for iter in 0..(w * h) {
            let xs = xs.iter().map(|(x, y, dx, dy)| {
                (((x + dx * iter) % w + w) % w, ((y + dy * iter) % h + h) % h)
            });
            let mut count = 0;
            for (x, y) in xs {
                if (x > 0 && hs[(x - 1) as usize][y as usize] == iter)
                    || (y > 0 && hs[x as usize][(y - 1) as usize] == iter)
                {
                    count += 1;
                }
                hs[x as usize][y as usize] = iter;
            }
            if iter > 0 && count > hi {
                hi = count;
                hi_it = iter;
            }
        }
        Ok(hi_it)
    }
}
