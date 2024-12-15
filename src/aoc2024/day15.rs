use std::fmt::Debug;

use anyhow::Result;

use crate::util::*;

const DEBUG_OUTPUT: bool = false;

pub struct Day15;

impl Solution for Day15 {
    type Input = (Vec<Vec<u8>>, Vec<(i8, i8)>);

    fn day() -> u8 {
        15
    }

    fn default_input() -> Result<Vec<u8>> {
        read_bytes!("inputs/aoc2024/day15.txt")
    }

    fn parse(input: &Vec<u8>) -> Result<Self::Input> {
        let lines = input.lines().collect_vec();
        let parts = lines.split(|x| x.len() == 0).collect_vec();
        let a = parts[0].iter().map(|x| x.clone()).collect_vec();
        let b = parts[1]
            .iter()
            .flatten()
            .map(|&x| match x {
                b'^' => (-1, 0),
                b'>' => (0, 1),
                b'v' => (1, 0),
                b'<' => (0, -1),
                _ => (0i8, 0i8),
            })
            .collect_vec();
        Ok((a, b))
    }

    fn p1((mut board, moves): Self::Input) -> Result<impl Debug> {
        let mut bi = 0;
        let mut bj = 0;
        'outer: for (i, l) in board.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'@' {
                    bi = i;
                    bj = j;
                    break 'outer;
                }
            }
        }
        board[bi][bj] = b'.';
        let si = board.len();
        let sj = board[0].len();
        'outer: for (mi, mj) in moves {
            let mi = mi as usize;
            let mj = mj as usize;
            let mut ni = bi + mi;
            let mut nj = bj + mj;
            loop {
                if ni == 0 || nj == 0 || ni == si || nj == sj || board[ni][nj] == b'#' {
                    continue 'outer;
                }
                if board[ni][nj] != b'O' {
                    break;
                }
                ni += mi;
                nj += mj;
            }
            bi += mi;
            bj += mj;
            if board[bi][bj] == b'O' {
                board[bi][bj] = b'.';
                board[ni][nj] = b'O';
            }
        }
        let mut sum = 0;
        for (i, l) in board.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'O' {
                    sum += i * 100 + j;
                }
            }
        }
        Ok(sum)
    }

    fn p2((board, moves): Self::Input) -> Result<impl Debug> {
        let mut board = board
            .iter()
            .map(|l| {
                l.iter()
                    .flat_map(|&x| match x {
                        b'#' => vec![b'#', b'#'],
                        b'O' => vec![b'[', b']'],
                        b'.' => vec![b'.', b'.'],
                        b'@' => vec![b'@', b'.'],
                        _ => Vec::new(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let mut bi = 0;
        let mut bj = 0;
        'outer: for (i, l) in board.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'@' {
                    bi = i;
                    bj = j;
                    break 'outer;
                }
            }
        }
        board[bi][bj] = b'.';
        let si = board.len();
        let sj = board[0].len();
        if DEBUG_OUTPUT {
            println!(
                "{}",
                board
                    .iter()
                    .map(|x| String::from_utf8_lossy(x))
                    .collect_vec()
                    .join("\n")
            );
        }
        for (mi, mj) in moves {
            let ni = bi + mi as usize;
            let nj = bj + mj as usize;
            if ni == 0 || nj == 0 || ni == si || nj == sj {
                continue;
            }
            if DEBUG_OUTPUT {
                let x = board[bi][bj];
                let y = board[ni][nj];
                board[bi][bj] = b'o';
                board[ni][nj] = b'x';
                println!(
                    "{}",
                    board
                        .iter()
                        .map(|x| String::from_utf8_lossy(x))
                        .collect_vec()
                        .join("\n")
                );
                board[bi][bj] = x;
                board[ni][nj] = y;
                println!(
                    "{:?} {} {} {}",
                    (ni, nj, mi, mj),
                    board[ni][nj] as char,
                    (mi != 0 || mj == -1) && board[ni][nj] == b']' && board[ni][nj - 1] == b'[',
                    (mi != 0 || mj == -1)
                        && board[ni][nj] == b']'
                        && can_move(&board, ni, nj - 1, mi, mj)
                );
            }
            if board[ni][nj] == b'.' {
                bi = ni;
                bj = nj;
            } else if board[ni][nj] == b'[' && can_move(&board, ni, nj, mi, mj) {
                do_move(&mut board, ni, nj, mi, mj);
                bi = ni;
                bj = nj;
            } else if (mi != 0 || mj == -1)
                && board[ni][nj] == b']'
                && can_move(&board, ni, nj - 1, mi, mj)
            {
                do_move(&mut board, ni, nj - 1, mi, mj);
                bi = ni;
                bj = nj;
            }
        }
        let mut sum = 0;
        for (i, l) in board.iter().enumerate() {
            for (j, &c) in l.iter().enumerate() {
                if c == b'[' {
                    sum += i * 100 + j;
                }
            }
        }
        if DEBUG_OUTPUT {
            board[bi][bj] = b'@';
            println!("bot is at: {:?}", (bi, bj));
            println!(
                "{}",
                board
                    .iter()
                    .map(|x| String::from_utf8_lossy(x))
                    .collect_vec()
                    .join("\n")
            );
        }
        Ok(sum)
    }
}

fn can_move(board: &Vec<Vec<u8>>, i: usize, j: usize, mi: i8, mj: i8) -> bool {
    let si = board.len();
    let sj = board[0].len();
    if let (Some(ni), Some(nj)) = (
        i.checked_add_signed(mi as isize),
        j.checked_add_signed(mj as isize),
    ) {
        if ni >= si || nj >= sj || board[ni][nj] == b'#' || board[ni][nj + 1] == b'#' {
            false
        } else {
            if mi != 0 {
                if board[ni][nj] == b'[' {
                    can_move(board, ni, nj, mi, mj)
                } else {
                    (board[ni][nj] != b']' || can_move(board, ni, nj - 1, mi, mj))
                        && (board[ni][nj + 1] != b'[' || can_move(board, ni, nj + 1, mi, mj))
                }
            } else if mj == -1 {
                board[ni][nj - 1] != b'[' || can_move(board, ni, nj - 1, mi, mj)
            } else {
                board[ni][nj + 1] != b'[' || can_move(board, ni, nj + 1, mi, mj)
            }
        }
    } else {
        false
    }
}

fn do_move(board: &mut Vec<Vec<u8>>, i: usize, j: usize, mi: i8, mj: i8) {
    if let (Some(ni), Some(nj)) = (
        i.checked_add_signed(mi as isize),
        j.checked_add_signed(mj as isize),
    ) {
        if mi != 0 {
            if board[ni][nj] == b'[' {
                do_move(board, ni, nj, mi, mj)
            }
            if board[ni][nj] == b']' {
                do_move(board, ni, nj - 1, mi, mj);
            }
            if board[ni][nj + 1] == b'[' {
                do_move(board, ni, nj + 1, mi, mj);
            }
        } else if mj == -1 {
            if board[ni][nj - 1] == b'[' {
                do_move(board, ni, nj - 1, mi, mj);
            }
        } else {
            if board[ni][nj + 1] == b'[' {
                do_move(board, ni, nj + 1, mi, mj);
            }
        }
        board[i][j] = b'.';
        board[i][j + 1] = b'.';
        board[ni][nj] = b'[';
        board[ni][nj + 1] = b']';
    }
}
