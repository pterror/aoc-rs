use std::time;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

fn time(day: i32, part: i32, callback: impl FnOnce()) {
    print!("d{day}p{part}: ");
    let start = time::SystemTime::now();
    callback();
    let delta = time::SystemTime::now().duration_since(start).unwrap();
    println!("      ({delta:?})");
}

pub fn run_all() {
    time(1, 1, || day1::p1().unwrap());
    time(1, 2, || day1::p2().unwrap());
    time(2, 1, || day2::p1().unwrap());
    time(2, 2, || day2::p2().unwrap());
    time(3, 1, || day3::p1().unwrap());
    time(3, 2, || day3::p2().unwrap());
    time(4, 1, || day4::p1().unwrap());
    time(4, 2, || day4::p2().unwrap());
    time(5, 1, || day5::p1().unwrap());
    time(5, 2, || day5::p2().unwrap());
    time(6, 1, || day6::p1().unwrap());
    time(6, 2, || day6::p2().unwrap());
    time(7, 1, || day7::p1().unwrap());
    time(7, 2, || day7::p2().unwrap());
}
