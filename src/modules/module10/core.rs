use super::{Number, NumberToFind, SolveResult, SolveError};

// ------------------------------------------------------------------------------------------------

/// Переводит все числа в `Vec` в числа с требуемым основанием
fn convert_all(numbers: &Vec<Number>, base: u32) -> Option<Vec<Number>> {
    let mut converted_numbers = vec![];

    for num in numbers {
        let num_res = num.convert(base);
        if let Err(_) = num_res { return None; }
        converted_numbers.push(num_res.unwrap());
    }

    Some(converted_numbers)
}

// ------------------------------------------------------------------------------------------------

/// Ищет среди чисел `numbers` минимальное или максимальное
pub fn find_num(numbers: &Vec<Number>, what_find: NumberToFind) -> SolveResult {
    let nums_in_decimal = convert_all(numbers, 10).ok_or(SolveError::UnableToSolve)?;

    Ok(match what_find {
        NumberToFind::Min => nums_in_decimal.iter().min(),
        NumberToFind::Max => nums_in_decimal.iter().max(),
    }.unwrap().number())
}

/// Обёртка над `Number::convert()`, возвращающая `SolveResult`, вместо `ConvertionResult`
pub fn convert(number: &Number, base: u32) -> SolveResult {
    match number.convert(base) {
        Ok(n)  => Ok(n.number()),
        Err(_) => Err(SolveError::UnableToSolve),
    }
}

/// Находит число с наибольшей/наименьшей суммой цифр
pub fn find_digits_sum(numbers: &Vec<Number>, base: u32, what_find: NumberToFind) -> SolveResult {
    let converted_numbers = convert_all(numbers, base).ok_or(SolveError::UnableToSolve)?;
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

/// Находит число с наибольшим/наименьшим количеством единиц
pub fn find_ones_count(numbers: &Vec<Number>, what_find: NumberToFind) -> SolveResult {
    let converted_numbers = convert_all(numbers, 2).ok_or(SolveError::UnableToSolve)?;
    let mut ones = vec![];

    for num in converted_numbers {
        ones.push(i32::from_str_radix(&num.number(), 2).unwrap().abs().count_ones());
    }

    Ok(match what_find {
        NumberToFind::Min => ones.iter().min(),
        NumberToFind::Max => ones.iter().max(),
    }.unwrap().to_string())
}

