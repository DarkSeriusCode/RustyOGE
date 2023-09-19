use rusty_oge::module2;
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use crate::utils::{CLIResult, Pair};
use super::input_utils::*;

pub fn get_input() -> CLIResult<module2::InputData> {
    let codes = get_codes()?;
    let encoded_strings = input_until_end("Введите строки")?;
    let spec = get_spec()?;

    let input_data = module2::InputData::new(codes, encoded_strings, spec);
    if let Err(validation_error_text) = input_data.valid() {
        return Err(CLIError::InvalidInputData(validation_error_text));
    }

    Ok(input_data)
}

fn get_codes() -> CLIResult<module2::Codes> {
    let mut codes = module2::Codes::new();

    for s in input_until_end::<Pair<String, String>>("Введите букву и её код через пробел")? {
        codes.insert(s.second().to_owned(), s.first().to_owned());
    }
    Ok(codes)
}

fn get_spec() -> CLIResult<module2::ProblemSpec> {
    // Если нужно только найти строку с одной расшифровкой, то, по умолчанию, символы могут
    // повторятся
    let only_decode = ask("Нужно найти строку, имеющую только одну расшифровку?")?;
    let mut only_unique_chars = false;
    let data_format = choose("Какие данные вывести?", &[
        ("Декодированную строку", &module2::OutputDataType::DecodedString),
        ("Длину строки",          &module2::OutputDataType::Length),
        ("Повторяющиеся символы", &module2::OutputDataType::RepeatingChars)
    ])?;
    if !only_decode {
        only_unique_chars = ask("В строке символы не должны повторяться?")?;
    }

    Ok(module2::ProblemSpec::new(only_decode, only_unique_chars, *data_format))
}
