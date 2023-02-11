use std::path::Path;

use crate::task6::core;
use crate::SolveResult;

pub const SOLVERS_COUNT: u8 = 1;

// --------------------------------------------------------------------------------------

/// Возвращает строку формата "u8;u8"
pub fn solve_type1(program_path: &Path, program_input: &String) -> SolveResult {
    let args = core::format_program_input(program_input);
    let mut right_answer_count = 0;

    for arg in args {
        let program_output = core::run_program(program_path, &arg)?;
        if program_output == "YES" {
            right_answer_count += 1;
        }
    }
    Ok(right_answer_count.to_string())
}
