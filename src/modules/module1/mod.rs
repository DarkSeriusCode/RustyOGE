use crate::utils::{self, SolveError, SolveResult, data_size::DataSize, Validated};

mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    if let Err(validation_err) = input_data.valid() {
        return Err(SolveError(validation_err.into()));
    }

    if (input_data.bits_in_char as f64 / 8.0f64).fract() != 0.0 {
        return Err(SolveError(format!("There're 8 bits in 1 byte! Not {}!",
                                      input_data.bits_in_char).into()));
    }
    let bytes_in_chars = input_data.bits_in_char / 8;

    match input_data.spec {
        ProblemSpec::FindWord(size) => {
            let needed_len = size / bytes_in_chars;
            // todo!() потому что на этапе валидации гарантируется, что при FindWord,
            // text = ConcreteText
            let InputText::ConcreteText(text) = input_data.text else { todo!() };
            let found_word = core::find_word_with_len(&utils::normalize_text(&text), needed_len);
            found_word.ok_or(SolveError("Cannot find such word".into()))
        },
        ProblemSpec::CalcTextSize(unit) => {
            let text_size = DataSize::bytes(input_data.text.chars_count() * bytes_in_chars);
            Ok(text_size.in_unit(unit).size().to_string())
        }
    }
}
