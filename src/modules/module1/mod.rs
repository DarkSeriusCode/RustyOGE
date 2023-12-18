use crate::utils::{self, SolveError, SolveResult, data_size::DataSize};

mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let bytes_in_chars = input_data.bits_in_char / 8;

    match input_data.spec {
        ProblemSpec::FindWord(size) => {
            let needed_len = size / bytes_in_chars;
            // todo!() потому что на этапе валидации гарантируется, что при FindWord,
            // text = ConcreteText
            let InputText::ConcreteText(text) = input_data.text else { todo!() };
            let found_word = core::find_word_with_len(&utils::normalize_text(&text), needed_len);
            found_word.ok_or(SolveError("Невозможно найти такое слово".into()))
        },
        ProblemSpec::CalcTextSize(unit) => {
            let text_size = DataSize::bytes(input_data.text.chars_count() * bytes_in_chars);
            Ok(text_size.in_unit(unit).size().to_string())
        }
    }
}
