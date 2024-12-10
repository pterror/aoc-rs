#![allow(dead_code)]

#[allow(unused_imports)]
use util::Solution;
mod aoc2024;
mod template;
mod util;

type T = aoc2024::day10::Day10;

fn main() {
    // let input = T::parse(&T::default_input().unwrap()).unwrap();
    // println!("{:?}", T::p2(input).unwrap());
    aoc2024::run_all();
}
