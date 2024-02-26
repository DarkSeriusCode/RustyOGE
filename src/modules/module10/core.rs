use super::{Number, SolveResult, SolveError};
use crate::utils::NumberToFind;

/// Обёртка над `Number::convert()`, возвращающая `SolveResult`, вместо `ConvertionResult`
pub fn convert(number: &Number, base: u32) -> SolveResult {
    match number.convert(base) {
        Ok(n)  => Ok(n.number()),
        Err(e) => Err(SolveError(e.into())),
    }
}

/// Переводит все числа в `Vec` в числа с требуемым основанием
fn convert_all(numbers: &Vec<Number>, base: u32) -> Result<Vec<Number>, SolveError> {
    let mut converted_numbers = vec![];

    for num in numbers {
        // let num_res = num.convert(base);
        // if let Err(_) = num_res { return None; }
        // converted_numbers.push(num_res.unwrap());
        converted_numbers.push(num.convert(base).map_err(|e| SolveError(e.into()))?);
    }

    Ok(converted_numbers)
}

// ------------------------------------------------------------------------------------------------

/// Ищет среди чисел `numbers` минимальное или максимальное в десятичной системе счисления
pub fn find_num(numbers: &Vec<Number>, what_find: NumberToFind) -> SolveResult {
    let nums_in_decimal = convert_all(numbers, 10)?;

    Ok(match what_find {
        NumberToFind::Min => nums_in_decimal.iter().min(),
        NumberToFind::Max => nums_in_decimal.iter().max(),
    }.unwrap().number())
}


/// Находит число с наибольшей/наименьшей суммой цифр в заданной системе счисления
pub fn find_digits_sum(numbers: &Vec<Number>, base: u32, what_find: NumberToFind) -> SolveResult {
    let converted_numbers = convert_all(numbers, base)?;
    let mut sums = vec![];

    for num in converted_numbers {
        let sum_of_digits = num.number().chars()
                              .filter(|c| c.is_digit(base))
                              .fold(0, |acc, c| acc + c.to_digit(base).unwrap());
        sums.push(sum_of_digits);
    }

    Ok(match what_find {
        NumberToFind::Min => sums.iter().min(),
        NumberToFind::Max => sums.iter().max(),
    }.unwrap().to_string())

}

/// Находит число, имеющее максимальное/минимальное количество единиц в двоичной системе счисления.
pub fn find_ones_count(numbers: &Vec<Number>, what_find: NumberToFind) -> SolveResult {
    let converted_numbers = convert_all(numbers, 2)?;
    let mut ones = vec![];

    for num in converted_numbers {
        ones.push(i32::from_str_radix(&num.number(), 2).unwrap().abs().count_ones());
    }

    Ok(match what_find {
        NumberToFind::Min => ones.iter().min(),
        NumberToFind::Max => ones.iter().max(),
    }.unwrap().to_string())
}
