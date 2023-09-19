use crate::utils::{self, SolveError, SolveResult};

mod core;
mod types;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let output_data_type = input_data.spec.output_data_type;

    // Ищем строку с одной расшифровкой. В input_data.encoded_strings больше одной строки
    if input_data.spec.one_decoding {
        for encoded_str in &input_data.encoded_strings {
            let decoded = core::decode(&input_data.codes, &encoded_str);
            if decoded.len() == 1 {
                let decoded_str = decoded.first().unwrap();
                return Ok(output_data_type.format(decoded_str));
            }
        }
    }

    // В input_data.encoded_strings ТОЛЬКО одна строка, т.к данные прошли валидацию
    let encoded_string = input_data.encoded_strings.first().unwrap();
    let decoded_strings = core::decode(&input_data.codes, encoded_string);
    let decoded_strings = Vec::from_iter(decoded_strings.iter()
                                            .filter(|str| utils::has_unique_chars(str) && 
                                                          input_data.spec.unique_chars));

    if decoded_strings.len() == 0 {
        return Err(SolveError::UnableToSolve);
    }
    let first_decoded_str = decoded_strings.first().unwrap();

    Ok(output_data_type.format(first_decoded_str))
}
