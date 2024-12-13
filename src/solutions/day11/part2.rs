use std::collections::HashMap;

const NUM_BLINKS: u64 = 75;

fn blink (num_stones: u64, num_blinks: u64) -> Vec<u64> {
    let mut stones: Vec<u64> = vec![num_stones];

    for _ in 0..num_blinks {
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

    stones
}

fn count_blinks (num_stones: u64, num_blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if num_blinks == 0 {
        return 1;
    }

    if let Some(entry) = cache.get(&(num_stones, num_blinks)) {
        return *entry;
    }

    let mut total: u64 = 0;
    let stones = blink(num_stones, 1);
    for v in stones.iter() {
        total += count_blinks(*v, num_blinks - 1, cache);
    }

    cache.insert((num_stones, num_blinks), total as u64);

    total
}

pub fn solution (input: &str) -> String {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let mut total: u64 = 0;
    for num in stones.iter() {
        total += count_blinks(*num, NUM_BLINKS, &mut cache);
    }

    total.to_string()
}
