use rusty_oge::module10;
use rusty_oge::module10::{InputData, ProblemSpec, Number};
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use super::input_utils::*;

const FINDNUM_TEXT: &str       = "Найти наибольшее/наименьшее число в десятичной системе счисления";
const CONVERT_TEXT: &str       = "Перевести число в другую систему счисления";
const FINDDIGITSSUM_TEXT: &str = "Найти число, сумма цифр которого наибольшая/наименьшая в\
                                  заданной системе счисления";
const FINDONESCOUNT: &str      = "Найти число с наименьшим/наибольшим числом единиц в двоичной\
                                  системе счисления";

enum ProblemSpecType {
    FindNum,
    Convert,
    FindDigitsSum,
    FindOnesCount,
}

pub fn get_input() -> Result<module10::InputData, CLIError> {
    let numbers = get_numbers()?;
    let spec = get_spec()?;

    let input_data = InputData::new(numbers, spec);

    if !input_data.is_valid() {
        return Err(CLIError::IncorrectInput);
    }

    Ok(input_data)
}

fn get_numbers() -> Result<Vec<Number>, CLIError> {
    let mut numbers = vec![];

    let raw_input = input_until_end("Введите число и его систему счисления через пробел.")?;

    for line in raw_input {
        let pair: Vec<&str> = line.split_whitespace().collect();
        if pair.len() != 2 { return Err(CLIError::IncorrectInput); }

        let num = pair[0];
        let base: u32 = pair[1].parse().map_err(|_| CLIError::IncorrectInput)?;

        numbers.push(Number::new(num, base).map_err(|_| CLIError::IncorrectInput)?);
    }

    Ok(numbers)
}

// FIXME: Сделать ввод ProblemSpec проще и понятнее
fn get_spec() -> Result<module10::ProblemSpec, CLIError> {
    let spec_type = choose("Что требуется сделать в задаче?", &[
            (FINDNUM_TEXT,       &ProblemSpecType::FindNum),
            (CONVERT_TEXT,       &ProblemSpecType::Convert),
            (FINDDIGITSSUM_TEXT, &ProblemSpecType::FindDigitsSum),
            (FINDONESCOUNT ,     &ProblemSpecType::FindOnesCount),
    ])?;

    match spec_type {
        &ProblemSpecType::FindNum       => Ok(ProblemSpec::FindNum(get_number_to_find()?)),
        &ProblemSpecType::Convert       => get_convert_spec(),
        &ProblemSpecType::FindDigitsSum => get_find_digits_sum_spec(),
        &ProblemSpecType::FindOnesCount => Ok(ProblemSpec::FindOnesCount(get_number_to_find()?)),
    }

}

// ------------------------------------------------------------------------------------------------

fn get_number_to_find() -> Result<module10::NumberToFind, CLIError> {
    choose("Найти", &[
            ("Минимальное число", &module10::NumberToFind::Min),
            ("Максимальное число", &module10::NumberToFind::Max),
    ]).map(|ok| ok.to_owned())
}

fn get_convert_spec() -> Result<ProblemSpec, CLIError> {
    Ok(ProblemSpec::Convert(input("В систему счисления с каким основанием нужно перевести число: ")?
        .parse::<u32>()
        .map_err(|_| CLIError::IncorrectInput)?))
}

fn get_find_digits_sum_spec() -> Result<ProblemSpec, CLIError> {
    let base = input("Основание системы счисления: ")?.parse().map_err(|_| CLIError::IncorrectInput)?;
    Ok(ProblemSpec::FindDigitsSum(base, get_number_to_find()?))
}

