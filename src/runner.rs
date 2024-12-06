use std::time::Duration;

pub struct Solution {
    pub day: [&'static str; 2],
    pub solution: fn(input: &str) -> String,
}

pub struct SolutionResult {
    pub time_elapsed: String,
    pub result: String,
}

pub fn exec_solution(solution: &Solution, test: bool) -> SolutionResult {
    let input = std::fs::read_to_string(
        format!(
            "./src/solutions/day{day}.input{test}.txt",
            day = solution.day[0],
            test = if test { ".test" } else { "" }
        ),
    )
    .expect("Could not read file.");

    let mut times: Vec<Duration> = Vec::new();

    for _ in 0..10 {
        let started = std::time::Instant::now();
        (solution.solution)(&input);
        times.push(started.elapsed());
    }

    let result = (solution.solution)(&input);
    
    times.sort();
    let elapsed = times.get(0).unwrap();

    let time_elapsed = if elapsed.as_millis() >= 10 {
        format!("{} ms", elapsed.as_millis())
    } else if elapsed.as_micros() >= 10 {
        format!("{} µs", elapsed.as_micros())
    } else {
        format!("{} ns", elapsed.as_nanos())
    };

    SolutionResult {
        time_elapsed,
        result,
    }
}
