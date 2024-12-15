use std::time::Duration;

use crate::util::*;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn run_all(count_parsing: bool) -> Duration {
    let mut duration = Duration::ZERO;
    duration += time_sol::<day1::Day1>(count_parsing);
    duration += time_sol::<day2::Day2>(count_parsing);
    duration += time_sol::<day3::Day3>(count_parsing);
    duration += time_sol::<day4::Day4>(count_parsing);
    duration += time_sol::<day5::Day5>(count_parsing);
    duration += time_sol::<day6::Day6>(count_parsing);
    duration += time_sol::<day7::Day7>(count_parsing);
    duration += time_sol::<day8::Day8>(count_parsing);
    duration += time_sol::<day9::Day9>(count_parsing);
    duration += time_sol::<day10::Day10>(count_parsing);
    duration += time_sol::<day11::Day11>(count_parsing);
    duration += time_sol::<day12::Day12>(count_parsing);
    duration += time_sol::<day13::Day13>(count_parsing);
    duration += time_sol::<day14::Day14>(count_parsing);
    duration += time_sol::<day15::Day15>(count_parsing);
    duration
}
