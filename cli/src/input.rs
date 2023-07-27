use std::path::PathBuf;

use rusty_oge::{module2, module6};
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use crate::input_utils::*;

pub fn read_module2_input() -> Result<module2::InputData, CLIError> {
    // Получаем символы и их коды
    let mut codes = module2::Codes::new();
    for s in input_until_end("Введите букву и код через пробел")? {
        let pair = Vec::from_iter(s.split_whitespace());
        if pair.len() != 2 {
            return Err(CLIError::IncorrectInput);
        }
        codes.insert(pair[1].to_string(), pair[0].to_string());
    }

    // Получаем закодированные строки
    let encoded_strings = input_until_end("Введите строки")?;

    // Получаем детали задачи
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

    let spec = module2::ProblemSpec::new(only_decode, only_unique_chars, *data_format);
    let input_data = module2::InputData::new(codes, encoded_strings, spec);
    if !input_data.is_valid() {
        return Err(CLIError::IncorrectInput);
    }

    Ok(input_data)
}

// ------------------------------------------------------------------------------------------------

pub fn read_module6_input() -> Result<module6::InputData, CLIError> {
    let path = input("Введите путь до файла с программой: ")?;
    let input_string = input("Введите данные для программы: ")?;
    let expected_output = input("Что должна вывести программа: ")?;

    let mut program_path_buf = PathBuf::new();
    program_path_buf.push(&path);

    let spec = module6::ProblemSpec::new(expected_output);
    let input_data = module6::InputData::new(&program_path_buf, &input_string, spec);

    if !input_data.is_valid() {
        return Err(CLIError::IncorrectInput);
    }

    Ok(input_data)
}

