pub fn solution (input: &str) -> String {
    let reports = input.lines().map(|line| {
        let mut report_values = line
            .split_whitespace()
            .map(|y| y.parse::<i32>().unwrap());
    
        let mut prev = report_values.next().unwrap();
        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut valid_differences = true;
    
        for cur in report_values {
            let diff = (cur - prev).abs();
            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }
            if cur > prev {
                all_decreasing = false;
            } else if cur < prev {
                all_increasing = false;
            }
            prev = cur;
        }
    
        valid_differences && (all_increasing || all_decreasing)
    }).collect::<Vec<bool>>();

    reports
        .into_iter()
        .filter(|&is_safe| is_safe)
        .count()
        .to_string()
}
