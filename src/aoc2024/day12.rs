use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

fn process(
    map: &[Vec<u8>],
    character: u8,
    i: usize,
    j: usize,
    seen: &mut [Vec<bool>],
) -> (usize, usize) {
    seen[i][j] = true;
    let mut area = 1;
    let mut perimeter = 0;
    let si = map.len();
    let sj = map[0].len();
    for (di, dj) in vec![(-1, 0), (0, -1), (0, 1), (1, 0)] {
        if let (Some(i2), Some(j2)) = (i.checked_add_signed(di), j.checked_add_signed(dj)) {
            if i2 >= si || j2 >= sj || map[i2][j2] != character {
                perimeter += 1;
                continue;
            }
            if seen[i2][j2] {
                continue;
            }
            let (new_area, new_perimeter) = process(map, character, i2, j2, seen);
            area += new_area;
            perimeter += new_perimeter;
        } else {
            perimeter += 1;
        }
    }
    (area, perimeter)
}

fn process_2(
    map: &[Vec<u8>],
    character: u8,
    i: usize,
    j: usize,
    seen: &mut [Vec<bool>],
) -> (usize, usize) {
    seen[i][j] = true;
    let mut area = 1;
    let mut perimeter = 0;
    let si = map.len();
    let sj = map[0].len();
    for (di, dj) in vec![(-1, 0), (0, -1), (0, 1), (1, 0)] {
        let is_neighbor =
            if let (Some(i2), Some(j2)) = (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                i2 < si && j2 < sj && map[i2][j2] == character
            } else {
                false
            };
        if !is_neighbor {
            let mut add_perimeter = true;
            for &(side_i, side_j) in vec![(-dj, di), (dj, -di)]
                .iter()
                .filter(|(i, j)| *i <= 0 && *j <= 0)
            {
                if let (Some(i2), Some(j2)) =
                    (i.checked_add_signed(side_i), j.checked_add_signed(side_j))
                {
                    if i2 >= si || j2 >= sj || map[i2][j2] != character {
                        continue;
                    }
                    if let (Some(i3), Some(j3)) =
                        (i2.checked_add_signed(di), j2.checked_add_signed(dj))
                    {
                        if i3 < si && j3 < sj && map[i3][j3] == character {
                            continue;
                        }
                        add_perimeter = false;
                    } else {
                        add_perimeter = false;
                    }
                }
            }
            if add_perimeter {
                perimeter += 1;
            }
        }
    }
    for (di, dj) in vec![(-1, 0), (0, -1), (0, 1), (1, 0)] {
        if let (Some(i2), Some(j2)) = (i.checked_add_signed(di), j.checked_add_signed(dj)) {
            if i2 >= si || j2 >= sj || map[i2][j2] != character {
                continue;
            }
            if seen[i2][j2] {
                continue;
            }
            let (new_area, new_perimeter) = process_2(map, character, i2, j2, seen);
            area += new_area;
            perimeter += new_perimeter;
        }
    }
    (area, perimeter)
}

pub struct Day12;

impl Solution for Day12 {
    type Input = Vec<Vec<u8>>;

    fn day() -> u8 {
        12
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day12.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        input.lines().collect_vec().ok()
    }

    fn p1(xs: Self::Input) -> Result<impl Debug> {
        let mut all_seen = xs
            .iter()
            .map(|l| l.iter().map(|_| false).collect_vec())
            .collect_vec();
        let mut total = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if all_seen[i][j] {
                    continue;
                }
                let (area, perimeter) = process(&xs, c, i, j, &mut all_seen);
                total += area * perimeter;
            }
        }
        Ok(total)
    }

    fn p2(xs: Self::Input) -> Result<impl Debug> {
        let mut all_seen = xs
            .iter()
            .map(|l| l.iter().map(|_| false).collect_vec())
            .collect_vec();
        let mut total = 0;
        for (i, l) in xs.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if all_seen[i][j] {
                    continue;
                }
                let (area, perimeter) = process_2(&xs, c, i, j, &mut all_seen);
                total += area * perimeter;
            }
        }
        Ok(total)
    }
}
