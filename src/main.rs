use clap::Parser;

/// Helps you to solve OGE tasks
#[derive(Parser)]
struct CLI {
    /// The number of the task
    number: u32,

    /// The type of the task
    #[arg(short, long)]
    type_num: u8,
}

mod input;

use rusty_oge::*;
use rusty_oge::SolveResult;

fn main() {
    let args = CLI::parse();
    match solve_by_num(args.number, args.type_num) {
        Ok(a) => println!("Ответ: {a}"),
        Err(err) => eprintln!("{err}"),
    }
}

fn solve_by_num(number: u32, type_num: u8) -> SolveResult {
    let answer = match number {
        2 => task2::solve(input::read_task2_input, type_num),
        _ => Err("Cannot find the task"),
    }?;
    Ok(answer)
}
