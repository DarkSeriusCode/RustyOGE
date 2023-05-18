use clap::Parser;
use std::boxed::Box;
use std::error::Error;
use rusty_oge::*;

/// Helps you to solve OGE tasks
#[derive(Parser)]
struct CLI {
    /// The number of the task
    #[arg(long)]
    problem_num: u32,
}

mod input;
mod errors;

fn main() {
    let args = CLI::parse();
    match solve_by_num(args.problem_num) {
        Ok(a) => println!("Ответ: {a}"),
        Err(err) => eprintln!("{err}"),
    }
}

fn solve_by_num(number: u32) -> Result<String, Box<dyn Error>> {
    match number {
        2 => Ok(module2::solve(input::read_module2_input()?)?),
        6 => Ok(module6::solve(input::read_module6_input()?)?),
        _ => Err(Box::new(errors::CLIError::UnknownProblem)),
    }
}
