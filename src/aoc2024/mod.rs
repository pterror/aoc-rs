use std::time;

use anyhow::Result;

use crate::util::Solution;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn time(day: u8, part: u8, callback: impl FnOnce() -> Result<String>) {
    let start = time::SystemTime::now();
    let str = callback().unwrap_or_else(|_| String::from("<error>"));
    let delta = time::SystemTime::now().duration_since(start).unwrap();
    println!("d{day}p{part}: {delta:?}\t{str}");
}

fn time_sol_helper<T: Solution>() -> Result<()> {
    let day = T::day();
    let default_input = T::default_input()?;
    time(day, 1, || {
        let input = T::parse(&default_input)?;
        Ok(format!("{:?}", T::p1(input)?))
    });
    time(day, 2, || {
        let input = T::parse(&default_input)?;
        Ok(format!("{:?}", T::p2(input)?))
    });
    Ok(())
}

fn time_sol<T: Solution>() {
    time_sol_helper::<T>().unwrap();
}

pub fn run_all() {
    time_sol::<day1::Day1>();
    time_sol::<day2::Day2>();
    time_sol::<day3::Day3>();
    time_sol::<day4::Day4>();
    time_sol::<day5::Day5>();
    time_sol::<day6::Day6>();
    time_sol::<day7::Day7>();
    time_sol::<day8::Day8>();
    time_sol::<day9::Day9>();
}
