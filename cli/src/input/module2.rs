use rusty_oge::module2;
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use super::input_utils::*;

pub fn get_input() -> Result<module2::InputData, CLIError> {
    // Получаем символы и их коды
    let mut codes = module2::Codes::new();
    for s in input_until_end("Введите букву и код через пробел")? {
        let pair = Vec::from_iter(s.split_whitespace());
        if pair.len() != 2 {
            // TODO: Rewrite error message
            return Err(CLIError::IncorrectInput(
                       format!("{:?} вы должны ввести ДВА значения", pair).into()
            ));
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
        return Err(CLIError::InvalidInputData);
    }

    Ok(input_data)
}
