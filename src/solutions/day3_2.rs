use regex::Regex;

pub fn solution(input: &str) -> String {
    let regex = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?<enable>do\(\))|(?<disable>don't\(\))").unwrap();

    let mut mul_enabled = true;
    
    regex
        .captures_iter(input)
        .map(|cap| {
            if let Some(_) = cap.name("enable") {
                mul_enabled = true;
                return 0;
            }
            if let Some(_) = cap.name("disable") {
                mul_enabled = false;
                return 0;
            }
            if let (Some(left), Some(right)) = (cap.get(1), cap.get(2)) {
                if mul_enabled {
                    let left_val = left.as_str().parse::<i32>().unwrap_or(0);
                    let right_val = right.as_str().parse::<i32>().unwrap_or(0);
                    return left_val * right_val;
                }
            }
            0
        })
        .sum::<i32>()
        .to_string()
}
