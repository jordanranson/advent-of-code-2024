use std::time::Duration;

// cargo crates
use clap::Parser;
use colored::*;

// local crates
mod solutions; 
use solutions::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Specify the day
    #[arg(short, long)]
    day: String,

    // Use test input if provided
    #[arg(short, long)]
    test: bool,

    // Show result value in console
    #[arg(short, long)]
    out: bool,
}

const OUTPUT_WIDTH: usize = 35;
const NUMBER_FORMATTING_SEP: &str = ",";

fn pad_right (input: &str, length: usize) -> String {
    let input_len = input.chars().count();
    if length <= input_len {
        return input.to_string();
    }

    let padding = " ".repeat(length - input_len);
    format!("{}{}", input, padding)
}

fn format_number (sep: &str, number: u128) -> String {
    number.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(sep)
}

fn format_time_elapsed (elapsed: &Duration) -> String {
    if elapsed.as_millis() >= 10 {
        return format!("{} ms", format_number(NUMBER_FORMATTING_SEP, elapsed.as_millis()));
    } else if elapsed.as_micros() >= 10 {
        return format!("{} µs", format_number(NUMBER_FORMATTING_SEP, elapsed.as_micros()));
    } else {
        return format!("{} ns", format_number(NUMBER_FORMATTING_SEP, elapsed.as_nanos()));
    }
}

fn exec_solution (day: &usize, solution: &Solution, test: bool) -> SolutionResult {
    let input = std::fs::read_to_string(
        format!(
            "./src/solutions/day{day}/input{test}.txt",
            day = day + 1,
            test = if test { ".test" } else { "" }
        ),
    )
    .expect("Could not read file.");

    let mut parts: Vec<SolutionResultPart> = Vec::new();

    for part in solution.parts.iter() {
        let mut times: Vec<Duration> = Vec::new();

        let num_loops = if test { 10 } else { 1 };
        for _ in 0..num_loops {
            let started = std::time::Instant::now();
            (part.solve)(&input);
            times.push(started.elapsed());
        }

        let result = (part.solve)(&input);
        
        times.sort();
        let time_elapsed = times.get(0).unwrap();

        parts.push(SolutionResultPart {
            time_elapsed: *time_elapsed,
            result,
        });
    }

    SolutionResult {
        parts,
    }
}

fn main () {
    let args = Args::parse();
    let day = args.day.parse::<usize>().unwrap() - 1;
    let found_solution = SOLUTIONS.get(day);

    println!("");
    println!(" ╔═════════════════════════════════════╗");
    let output_header = format!("Advent of Code '24 - Day {}", day + 1);
    println!(" ║ {} ║", pad_right(&output_header, OUTPUT_WIDTH).bold());
    if args.test {
        println!(" ║ {} ║", pad_right(">> using test input", OUTPUT_WIDTH).bright_yellow());
    }

    let mut total_time_elapsed = Duration::new(0, 0);

    if let Some(solution) = found_solution {
        let solution_result = exec_solution(&day, &solution, args.test);

        solution_result.parts.iter().enumerate().for_each(|(index, part)| {
            total_time_elapsed += part.time_elapsed;

            println!(" ╟─── part {} ──────────────────────────╢", index + 1);
            
            let output_time = format!(
                "time       -> {}",
                format_time_elapsed(&part.time_elapsed).bright_green()
            );
            println!(" ║ {} ║", pad_right(&output_time, OUTPUT_WIDTH + 9));
            
            if args.out {
                let output_result = format!(
                    "result     -> {}",
                    part.result.bright_green()
                );
                println!(" ║ {} ║", pad_right(&output_result, OUTPUT_WIDTH + 9));
            }
        });
    } else {
        eprintln!(" ║ {} ║", pad_right("Error: Solution not found", OUTPUT_WIDTH).bright_red());
    }

    println!(" ╟─────────────────────────────────────╢");
    let total_time = format!(
        "total time -> {}",
        format_time_elapsed(&total_time_elapsed).bright_green()
    );
    println!(" ║ {} ║", pad_right(&total_time, OUTPUT_WIDTH + 9));
    println!(" ╚═════════════════════════════════════╝");
    println!();
}
