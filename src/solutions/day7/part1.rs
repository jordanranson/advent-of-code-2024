pub fn test_operations (numbers: Vec<i128>, result: i128) -> i128 {
    let num_operators = numbers.len() - 1;
    let total_combinations = 2usize.pow(num_operators as u32);

    for combination in 0..total_combinations {
        let mut total = numbers[0];
        let mut index = combination;

        for &num in &numbers[1..] {
            if index & 1 == 0 {
                total += num;
            } else {
                total *= num;
            }
            index >>= 1;
        }

        if total == result { 
            return result;
        }
    }

    0
}

pub fn solution (input: &str) -> String {
    input.lines()
        .map(|line| {
            let parts = line
                .split(":")
                .collect::<Vec<&str>>();

            let result = parts[0]
                .trim()
                .parse::<i128>()
                .unwrap();

            let numbers = parts[1]
                .trim()
                .split_whitespace()
                .map(|number| number.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();

            test_operations(numbers, result)
        })
        .sum::<i128>()
        .to_string()
}
