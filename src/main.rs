use clap::Parser;
use colored::*;

mod runner;
use runner::*;

mod solutions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Specify the day and part as `day,part` (e.g., "1,1")
    #[arg(short, long)]
    day: String,

    // Use test input if provided
    #[arg(short, long)]
    test: bool,

    // Show result value in console
    #[arg(short, long)]
    result: bool,
}

const SOLUTIONS: [Solution; 4] = [
    Solution {
        day: ["1", "1"],
        solution: solutions::day1_1::solution,
    },
    Solution {
        day: ["1", "2"],
        solution: solutions::day1_2::solution,
    },
    Solution {
        day: ["2", "1"],
        solution: solutions::day2_1::solution,
    },
    Solution {
        day: ["2", "2"],
        solution: solutions::day2_2::solution,
    },
];

const OUTPUT_WIDTH: usize = 40;

fn pad_right(input: &str, length: usize) -> String {
    let padding = " ".repeat(length - input.chars().count());
    format!("{}{}", input, padding)
}

fn main() {
    let args = Args::parse();

    let day_parts: Vec<&str> = args.day.split(',').collect();

    let found_solution = SOLUTIONS.iter().find(|solution| {
        solution.day[0] == day_parts[0] && solution.day[1] == day_parts[1]
    });

    println!("");
    println!(" ╔══════════════════════════════════════════╗");

    let output_header = format!(
        ">> Day {day}, Part {part}",
        day = day_parts[0],
        part = day_parts[1],
    );
    println!(" ║ {} ║", pad_right(&output_header, OUTPUT_WIDTH).bold());

    println!(" ╟──────────────────────────────────────────╢");

    if let Some(solution) = found_solution {
        let solution_result = exec_solution(solution, args.test);

        if args.test {
            println!(" ║ {} ║", pad_right("Warning: Using test input", OUTPUT_WIDTH).bright_yellow());
        }

        let output_time = format!(
            "time   -> {}",
            solution_result.time_elapsed.bright_green()
        );
        println!(" ║ {} ║", pad_right(&output_time, OUTPUT_WIDTH + 9));
        
        if args.result {
            let output_result = format!(
                "result -> {}",
                solution_result.result.bright_green()
            );
            println!(" ║ {} ║", pad_right(&output_result, OUTPUT_WIDTH + 9));
        }
    } else {
        eprintln!(" ║ {} ║", pad_right("Error: Solution not found", OUTPUT_WIDTH).bright_red());
    }

    println!(" ╚══════════════════════════════════════════╝");
    println!();
}
