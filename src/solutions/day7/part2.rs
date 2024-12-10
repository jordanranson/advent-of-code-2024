pub fn test_operations (numbers: Vec<i128>, result: i128) -> i128 {
    let num_operators = numbers.len() - 1;
    let total_combinations = 3usize.pow(num_operators as u32);

    for combination in 0..total_combinations {
        let mut total = numbers[0];
        let mut index = combination;

        for i in 1..numbers.len() {
            match index % 3 {
                0 => total += numbers[i],
                1 => total *= numbers[i],
                2 => {
                    total = format!("{}{}", total, numbers[i])
                        .parse::<i128>()
                        .unwrap();
                }
                _ => unreachable!(),
            }
            index /= 3;
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
