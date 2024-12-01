use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod day1;

#[derive(Parser)]
#[command(name = "advent-of-code-2024")]
#[command(about = "Calculate solutions for Advent of Code 2024 using Rust and the provided input", long_about = None)]
struct Cli {
    input_path: String,
    #[arg(long)]
    // TODO: Make this parameter optional and if not provided, solve all days.
    // day: Option<u8>
    day: u8,
    #[arg(long)]
    // TODO: Make this parameter optional and if not provided, solve all parts of a given day.
    // part: Option<u8>
    part: u8,
    #[arg(long, short, action)]
    // debug: bool
    debug: bool,
}

fn load_file(filename: PathBuf) -> Option<std::string::String> {
    let input_filename = filename.as_path().display().to_string();

    match fs::read_to_string(filename) {
        Ok(path) => Some(path),
        Err(err) => {
            println!("Could not load input file '{}'. {}", input_filename, err);
            None
        }
    }
}

type SolverFn = fn(&str) -> Result<String, String>;

fn solve(day: u8, part: u8) -> Result<SolverFn, (u8, u8)> {
    match (day, part) {
        (1, 1) => Ok(day1::part1),
        (1, 2) => Ok(day1::part2),
        (_, _) => Err((day, part)),
    }
}

fn main() {
    let cli = Cli::parse();
    let chrono_start;
    let chrono_stop;
    let solution_result;
    let mut total_time: u128 = 0;

    if cli.debug {
        Builder::new().filter_level(LevelFilter::Debug).init();
    } else {
        Builder::new().filter_level(LevelFilter::Info).init();
    }

    let day_input: PathBuf = [cli.input_path, format!("day{}.txt", cli.day)]
        .iter()
        .collect();

    if let Some(puzzle_input) = load_file(day_input) {
        match solve(cli.day, cli.part) {
            Ok(solve_function) => {
                chrono_start = Instant::now();
                solution_result = solve_function(&puzzle_input);
                chrono_stop = chrono_start.elapsed().as_micros();
                total_time += chrono_stop;

                match solution_result {
                    Ok(solution) => println!(
                        "Solution of Day {}, Part {}: {}, Time: {}μs",
                        cli.day, cli.part, solution, chrono_stop
                    ),
                    Err(error) => println!(
                        "A problem occured to solve the problem of Day {}, Part {}: {}, Time: {}μs",
                        cli.day, cli.part, error, chrono_stop
                    ),
                }
                println!("\nTotal Time: {}μs", total_time);
            }
            Err(_) => println!("Unsupported day {} and part {}", cli.day, cli.part),
        }
    }
}
