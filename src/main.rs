use clap::Parser;

/// Helps you to solve OGE tasks
#[derive(Parser)]
struct CLI {
    /// The number of the task
    #[arg(long)]
    task_num: u32,

    /// The type of the task
    #[arg(long)]
    task_type: u8,
}

mod input;

use rusty_oge::*;
use rusty_oge::SolveResult;

fn main() {
    let args = CLI::parse();
    match solve_by_num(args.task_num, args.task_type) {
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
