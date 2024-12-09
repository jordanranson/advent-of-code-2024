use regex::Regex;

pub fn solution(input: &str) -> String {
    let regex = Regex::new(r"(?:mul\((\d+),(\d+)\))").unwrap();

    regex
        .captures_iter(input)
        .map(|cap| 
            cap[1].parse::<i32>().unwrap_or(0) * 
            cap[2].parse::<i32>().unwrap_or(0)
        )
        .sum::<i32>()
        .to_string()
}
