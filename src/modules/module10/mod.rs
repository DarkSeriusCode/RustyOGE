use crate::utils::{SolveError, SolveResult};

mod types;
mod core;
mod number;

pub use types::*;
pub use number::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let first_number = input_data.numbers.first().unwrap();

    use ProblemSpec::*;
    let answer = match input_data.spec {
        FindNum(what)             => core::find_num(&input_data.numbers, what),
        Convert(base)             => core::convert(first_number, base),
        FindDigitsSum(base, what) => core::find_digits_sum(&input_data.numbers, base, what),
        FindOnesCount(what)       => core::find_ones_count(&input_data.numbers, what),
    }?;

    Ok(answer)
}
