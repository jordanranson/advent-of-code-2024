pub fn solution (input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let num: i32 = line.parse().unwrap();
        sum += num;
    }
    sum.to_string()
}
