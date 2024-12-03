pub fn solution (input: &str) -> String {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    for line in input.lines() {
        let pair = line
            .trim()
            .split("   ")
            .map(|side| side.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        left_side.push(pair[0]);
        right_side.push(pair[1]);
    }

    let mut sum: i32 = 0;

    for i in 0..left_side.len() {
        let mut occurrences: i32 = 0;
        for &num in &right_side {
            if num == left_side[i] {
                occurrences += 1;
            }
        }
        sum += left_side[i] * occurrences;
    }
    
    sum.to_string()
}
