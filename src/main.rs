use util::Solution;
mod aoc2024;
mod template;
mod util;

fn main() {
    // type T = aoc2024::day16::Day16;
    // let input = &T::default_input().unwrap();
    // println!("{:?}", T::run_p1(input).unwrap());
    // println!("{:?}", T::run_p2(input).unwrap());
    println!("total runtime (with parsing): {:?}", aoc2024::run_all(true));
    println!("");
    println!("total runtime (no parsing): {:?}", aoc2024::run_all(false));
}
