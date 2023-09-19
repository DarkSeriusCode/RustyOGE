use crate::utils::{SolveError, SolveResult};

mod core;
mod consts;
mod types;

pub use types::*;
use consts::ALT_CHARS;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    // Поиск Python
    let python_interpreter = match core::find_python() {
        Some(cmd) => cmd,
        None => return Err(SolveError::Other("Не найден Python!".to_string())),
    };

    let ascii_program_input = replace_unicode_chars(&input_data.program_input);
    let program_args: Vec<Vec<String>> = core::convert_program_input(&ascii_program_input);
    let mut correct_output_count = 0;

    // Запуск программы
    for args in &program_args {
        match core::run_program(&python_interpreter, &input_data.file_path, args) {
            Ok(output) if output == input_data.spec.expected_output => correct_output_count += 1,
            Ok(_) => (),
            Err(e) => return Err(SolveError::Other(format!("Ошибка запуска программы: {e}"))),
        }
    }

    Ok(correct_output_count.to_string())
}

// ------------------------------------------------------------------------------------------------

/// Во входных данных иногда может встретиться несколько Unicode символов для обозначения одного и
/// того же знака. Поэтому заменяем их на ASCII
fn replace_unicode_chars(string: &str) -> String {
    let mut string = string.to_string();
    for (unicode_ch_list, ch) in ALT_CHARS {
        for unicode_ch in unicode_ch_list {
            string = string.replace(*unicode_ch, &ch.to_string());
        }
    }
    string.to_string()
}
