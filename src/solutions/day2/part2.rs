pub fn solution(input: &str) -> String {
    let reports = input.lines().map(|line| {
        let report_values: Vec<i32> = line
            .split_whitespace()
            .map(|y| y.parse::<i32>().unwrap())
            .collect();

        let is_safe = |values: &[i32]| {
            let mut prev = values[0];
            let mut all_increasing = true;
            let mut all_decreasing = true;
            let mut valid_differences = true;

            for &cur in &values[1..] {
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
        };

        if is_safe(&report_values) {
            true
        } else {
            (0..report_values.len()).any(|i| {
                let mut modified = report_values.clone();
                modified.remove(i);
                is_safe(&modified)
            })
        }
    });

    reports
        .filter(|&is_safe| is_safe)
        .count()
        .to_string()
}
