use std::boxed::Box;
use std::error::Error;
use std::process::exit;

use clap::Parser;
use colored::Colorize;

use rusty_oge::*;

mod input;
mod errors;
mod utils;

const AVAILABLE_PROBLEMS: [u32;5] = [1, 2, 6, 10, 12];

// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
struct CLI {
    /// The number of the problem to be solved
    #[arg(long, short)]
    problem_num: Option<u32>,

    /// Show a list of available problems that can be solved and exit
    #[arg(long, short)]
    list: bool,
}

// ------------------------------------------------------------------------------------------------

fn main() {
    let args = CLI::parse();

    if args.list {
        show_list_of_available_problems();
        exit(0);
    }

    if let None = args.problem_num {
        println!("Program usage: {}", "rusty_oge-cli --help".bold());
        exit(0);
    }

    match solve_by_num(args.problem_num.unwrap()) {
        Ok(a) => println!("\n{}: {}", "Ответ".bold(), a.bold().green()),
        Err(err) => eprintln!("{}", err.to_string().bold().red()),
    }
}

// ------------------------------------------------------------------------------------------------

fn solve_by_num(number: u32) -> Result<String, Box<dyn Error>> {
    match number {
        1  => module1::solve(input::module1::get_input()?).map_err(|e| e.into()),
        2  => module2::solve(input::module2::get_input()?).map_err(|e| e.into()),
        6  => module6::solve(input::module6::get_input()?).map_err(|e| e.into()),
        10 => module10::solve(input::module10::get_input()?).map_err(|e| e.into()),
        12 => module12::solve(input::module12::get_input()?).map_err(|e| e.into()),
        n  => Err(errors::CLIError::UnknownProblem(n).into()),
    }
}

fn show_list_of_available_problems() {
    for problem_num in 1..=12 {
        let text = format!("Problem {problem_num}");

        if AVAILABLE_PROBLEMS.contains(&problem_num) {
            println!("{}", &text.green().bold().underline());
        } else {
            println!("{}", &text.red());
        }
    }
}
