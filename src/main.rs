use clap::Parser;

/// Helps you to solve OGE tasks
#[derive(Parser)]
struct CLI {
    /// The number of the task
    number: usize
}

pub mod tasks;
pub use tasks::task2;

fn main() {
    let args = CLI::parse();

    solve_by_num(args.number).unwrap_or_else(|err| eprintln!("{err}"));
}

fn solve_by_num(number: usize) -> Result<(), &'static str> {
    match number {
        2 => Ok(task2::solve()?),
        _ => Err("Cannot find the task"),
    }
}
