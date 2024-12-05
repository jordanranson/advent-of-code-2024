fn parse_input (char: char, input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(char).map(|x| x.trim().parse::<i32>().unwrap()).collect())
        .collect()
}

fn is_valid_update (rules: &[Vec<i32>], pages: &[i32]) -> bool {
    for rule in rules {
        let (page_a, page_b) = (rule[0], rule[1]);
        let index_a = pages.iter().position(|&p| p == page_a);
        let index_b = pages.iter().position(|&p| p == page_b);

        if let (Some(a), Some(b)) = (index_a, index_b) {
            if a > b {
                return false;
            }
        }
    }
    true
}

pub fn solution(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_input('|', parts[0]);
    let updates = parse_input(',', parts[1]);

    updates
        .iter()
        .filter(|pages| is_valid_update(&rules, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum::<i32>()
        .to_string()
}
