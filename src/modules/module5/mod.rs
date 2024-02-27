use crate::utils::{SolveResult, SolveError, Validated};

mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    if let Err(validation_err) = input_data.valid() {
        return Err(SolveError(validation_err.into()));
    }

    use ProblemSpec::*;
    match input_data.spec {
        FindVariableValue(program) => core::find_variable_value(&input_data.commands, &program,
                                                       input_data.begin_num, input_data.result_num)
            .ok_or(SolveError("Cannot find a variable value".into()))
            .map(|i| i.to_string()),
        MakeAlgorithm(len) => core::make_algorithm(&input_data.commands, len, input_data.begin_num,
                                                   input_data.result_num)
            .ok_or(SolveError("Cannot make algorithm!".into())),
    }
}
