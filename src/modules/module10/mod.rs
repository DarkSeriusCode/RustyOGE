mod types;
mod core;
mod number;

pub use types::*;
pub use number::*;

use crate::utils::{SolveError, SolveResult};

// ------------------------------------------------------------------------------------------------

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let first_number = input_data.numbers.first().unwrap();

    let answer = match input_data.spec {
        ProblemSpec::FindNum(what) => core::find_num(&input_data.numbers, what),
        ProblemSpec::Convert(base) => core::convert(first_number, base),
        ProblemSpec::FindDigitsSum(base, what) => core::find_digits_sum(&input_data.numbers, base, what),
        ProblemSpec::FindOnesCount(what) => core::find_ones_count(&input_data.numbers, what),
    }?;

    Ok(answer)
}

