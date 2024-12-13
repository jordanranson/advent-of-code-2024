use super::{Solution, SolutionPart};

mod part1;
mod part2;

pub const SOLUTION: Solution = Solution {
    parts: [
        SolutionPart { solve: part1::solution }, 
        SolutionPart { solve: part2::solution }, 
    ],
};
