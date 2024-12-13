const NUM_BLINKS: u64 = 25;

pub fn solution (input: &str) -> String {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..NUM_BLINKS {
        let mut new_stones = Vec::new();

        for stone in stones {
            if stone == 0 {
                // rule 1: replace 0 with 1
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                // rule 2: split the number into two stones
                let stone_str = stone.to_string();
                let mid = stone_str.len() / 2;
                let left = stone_str[..mid].parse::<u64>().unwrap();
                let right = stone_str[mid..].parse::<u64>().unwrap();
                new_stones.push(left);
                new_stones.push(right);
            } else {
                // rule 3: multiply the stone by 2024
                new_stones.push(stone * 2024);
            }
        }

        stones = new_stones;
    }

    stones.len().to_string()
}
