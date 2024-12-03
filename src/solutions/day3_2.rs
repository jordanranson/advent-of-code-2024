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
            if let (Some(_), Some(_)) = (cap.get(1), cap.get(2)) {
                if mul_enabled {
                    return
                        cap[1].parse::<i32>().unwrap_or(0) *
                        cap[2].parse::<i32>().unwrap_or(0);
                }
            }
            0
        })
        .sum::<i32>()
        .to_string()
}
