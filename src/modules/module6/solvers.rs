use std::path::Path;

use crate::module6::core;
use crate::module6::types::ProgramInput;
use crate::utils::{SolveResult, SolveError};

pub const SOLVERS_COUNT: u8 = 1;

// --------------------------------------------------------------------------------------

fn get_program_outputs(path: &Path, args: &Vec<ProgramInput>) -> Result<Vec<String>, SolveError> {
    let mut outputs = Vec::new();

    for arg in args {
        match core::run_program(path, &arg) {
            Ok(answer) => outputs.push(answer),
            Err(_) => return Err(SolveError::Other("Не могу запустить программу!".to_string())),
        }
    }

    Ok(outputs)
}

// --------------------------------------------------------------------------------------

/// Решает первый тип задачи: считает сколько раз программа вывела 'expected_out'
pub fn solve_type1(prog_path: &Path, prog_input: &str, expected_out: &str) -> SolveResult {
    let args = core::format_program_input(prog_input);
    let outputs = get_program_outputs(prog_path, &args)?;
    Ok(outputs.iter().filter(|i| *i == expected_out).count().to_string())
}

