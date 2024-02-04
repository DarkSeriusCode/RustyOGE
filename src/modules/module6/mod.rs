use crate::utils::{self, SolveError, SolveResult};

mod core;
mod consts;
mod types;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    // Поиск Python
    let python_interpreter = core::find_python()?;

    let ascii_program_input = utils::normalize_text(&input_data.program_input);
    let program_args: Vec<Vec<String>> = core::convert_program_input(&ascii_program_input);
    let mut correct_output_count = 0;

    // Запуск программы
    for args in &program_args {
        match core::run_program(&python_interpreter, &input_data.file_path, args) {
            Ok(output) if output == input_data.expected_output => correct_output_count += 1,
            Ok(_) => (),
            Err(e) => return Err(SolveError(format!("Cannot start the program ({e})").into())),
        }
    }

    Ok(correct_output_count.to_string())
}
