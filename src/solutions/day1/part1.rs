pub fn solution (input: &str) -> String {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut pair = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        left_side.push(pair.next().unwrap());
        right_side.push(pair.next().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let mut sum: i32 = 0;

    for i in 0..left_side.len() {
        sum += (left_side[i] - right_side[i]).abs();
    }
    
    sum.to_string()
}
