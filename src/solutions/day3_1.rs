use regex::Regex;

pub fn solution (input: &str) -> String {
    let matches = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            vec![
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            ]
        })
        .collect::<Vec<Vec<i32>>>();

    matches
        .iter()
        .map(|pair| pair[0] * pair[1])
        .sum::<i32>()
        .to_string()
}
