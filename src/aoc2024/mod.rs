use std::time;

use anyhow::Result;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

fn time(day: i32, part: i32, callback: impl FnOnce() -> Result<String>) {
    let start = time::SystemTime::now();
    let _ = callback();
    let delta = time::SystemTime::now().duration_since(start).unwrap();
    println!("d{day}p{part}: {delta:?}");
}

pub fn run_all() {
    time(1, 1, day1::p1);
    time(1, 2, day1::p2);
    time(2, 1, day2::p1);
    time(2, 2, day2::p2);
    time(3, 1, day3::p1);
    time(3, 2, day3::p2);
    time(4, 1, day4::p1);
    time(4, 2, day4::p2);
    time(5, 1, day5::p1);
    time(5, 2, day5::p2);
    time(6, 1, day6::p1);
    time(6, 2, day6::p2);
    time(7, 1, day7::p1);
    time(7, 2, day7::p2);
    time(8, 1, day8::p1);
    time(8, 2, day8::p2);
}
