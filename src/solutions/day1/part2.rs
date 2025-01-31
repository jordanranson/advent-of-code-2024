use std::collections::HashMap;

pub fn solution (input: &str) -> String {
    let mut left_side = Vec::new();
    let mut right_side = HashMap::new();

    for line in input.lines() {
        let mut pair = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        left_side.push(pair.next().unwrap());

        *right_side
            .entry(pair.next().unwrap())
            .or_insert(0) += 1;
    }

    let sum: i32 = left_side
        .iter()
        .filter_map(|&value| right_side.get(&value).map(|&count| value * count))
        .sum();

    sum.to_string()
}
