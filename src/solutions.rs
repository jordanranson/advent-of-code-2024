use std::time::*;

pub struct Solution {
    pub parts: [SolutionPart; 2],
}

pub struct SolutionPart {
    pub solve: fn (input: &str) -> String,
}

pub struct SolutionResult {
    pub parts: Vec<SolutionResultPart>,
}

pub struct SolutionResultPart {
    pub time_elapsed: Duration,
    pub result: String,
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const SOLUTIONS: [Solution; 9]= [
    day1::SOLUTION,
    day2::SOLUTION,
    day3::SOLUTION,
    day4::SOLUTION,
    day5::SOLUTION,
    day6::SOLUTION,
    day7::SOLUTION,
    day8::SOLUTION,
    day9::SOLUTION,
];
