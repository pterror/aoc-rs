#![allow(dead_code)]

#[allow(unused_imports)]
use util::Solution;
mod aoc2024;
mod template;
mod util;

type T = aoc2024::day11::Day11;

fn main() {
    // let input = T::parse(&T::default_input().unwrap()).unwrap();
    // println!("{:?}", T::p2(input).unwrap());
    println!("total runtime (with parsing): {:?}", aoc2024::run_all(true));
    println!("");
    println!("total runtime (no parsing): {:?}", aoc2024::run_all(false));
}
