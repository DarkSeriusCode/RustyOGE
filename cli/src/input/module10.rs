use rusty_oge::module10;
use rusty_oge::module10::{InputData, ProblemSpec, Number};
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use crate::utils::CLIResult;
use super::input_utils::*;

const FINDNUM_TEXT: &str       = "Найти наибольшее/наименьшее число в десятичной системе счисления";
const CONVERT_TEXT: &str       = "Перевести число в другую систему счисления";
const FINDDIGITSSUM_TEXT: &str = "Найти число, сумма цифр которого наибольшая/наименьшая в\
                                  заданной системе счисления";
const FINDONESCOUNT_TEXT: &str = "Найти число с наименьшим/наибольшим числом единиц в двоичной\
                                  системе счисления";

const SPEC_OPTIONS: [(&'static str, &dyn Fn() -> Result<ProblemSpec, CLIError>);4] = [
    (FINDNUM_TEXT,       &|| Ok(ProblemSpec::FindNum(get_number_to_find()?))),
    (CONVERT_TEXT,       &|| Ok(ProblemSpec::Convert(get_base()?))),
    (FINDDIGITSSUM_TEXT, &|| Ok(ProblemSpec::FindDigitsSum(get_base()?, get_number_to_find()?))),
    (FINDONESCOUNT_TEXT, &|| Ok(ProblemSpec::FindOnesCount(get_number_to_find()?))),
];

pub fn get_input() -> CLIResult<module10::InputData> {
    let numbers = get_numbers()?;
    let spec = choose("Что требуется сделать в задаче?", &SPEC_OPTIONS)?()?;

    let input_data = InputData::new(numbers, spec);

    if !input_data.is_valid() {
        return Err(CLIError::InvalidInputData);
    }

    Ok(input_data)
}

fn get_numbers() -> CLIResult<Vec<Number>> {
    let mut numbers = vec![];

    let raw_input = input_until_end("Введите число и его систему счисления через пробел.")?;

    for line in raw_input {
        let pair: Vec<&str> = line.split_whitespace().collect();
        if pair.len() != 2 {
            return Err(CLIError::IncorrectInput(
                    format!("{:?} вы должны ввести ДВА значения", pair).into()
            ));
        }

        let num = pair[0];
        let base: u32 = pair[1].parse().map_err(|_| CLIError::IncorrectInput(
                format!("{} не является числом!", pair[1]).into()
        ))?;

        numbers.push(Number::new(num, base).map_err(|e| CLIError::IncorrectInput(e.into()))?);
    }

    Ok(numbers)
}

// ------------------------------------------------------------------------------------------------

fn get_number_to_find() -> CLIResult<module10::NumberToFind> {
    choose("Найти", &[
            ("Минимальное число", &module10::NumberToFind::Min),
            ("Максимальное число", &module10::NumberToFind::Max),
    ]).map(|ok| ok.to_owned())
}

fn get_base() -> CLIResult<u32> {
    input("Введите основание системы счисления: ")
}

