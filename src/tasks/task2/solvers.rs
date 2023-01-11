// Про типы задач можно посмотреть в README.md
use std::collections::HashSet;
use crate::SolveResult;
use crate::task2::types::Codes;
use crate::task2::core::decode;

const UNABLE_TO_SOLVE_ERR_MSG: &str = "Не могу решить задачу!";

// --------------------------------------------------------------------------------------

fn has_unique_chars(string: &String) -> bool {
    let set: HashSet<char> = HashSet::from_iter(string.chars());
    set.len() == string.chars().count()
}

fn get_recurring_chars(string: &String) -> Option<Vec<String>> {
    let mut recurring_chars = Vec::new();
    for (ind, ch) in string.chars().enumerate() {
        if string.chars().skip(ind + 1).collect::<String>().contains(ch) {
            recurring_chars.push(ch.to_string());
        }
    }

    match recurring_chars.len() {
        1.. => Some(recurring_chars),
        _ => None,
    }
}

// --------------------------------------------------------------------------------------

pub fn solve_type1(codes: Codes, input: Vec<String>) -> SolveResult {
    for string in input {
        let decoded = decode(&codes, &string);
        // Если только одно решение, то и возвращаем его
        if decoded.len() == 1 {
            let s = &decoded[0];
            return Ok(s.to_string());
        }
    }
    Err(UNABLE_TO_SOLVE_ERR_MSG)
}

// --------------------------------------------------------------------------------------

pub fn solve_type2(codes: Codes, input: Vec<String>) -> SolveResult {
    let decoded = match input.get(0) {
        Some(s) => decode(&codes, s),
        None => return Err("Нет входных данных"),
    };

    // Отсееваем результаты, где повторяются символы
    let decoded = Vec::from_iter(decoded.iter().filter(|i| has_unique_chars(i)));

    // В decoded после фильтра ДОЛЖЕН остаться 1 элемент,
    // его и пытаемся вернуть
    match decoded.get(0) {
        Some(s) => Ok(s.to_string()),
        None => return Err(UNABLE_TO_SOLVE_ERR_MSG),
    }
}

// --------------------------------------------------------------------------------------

pub fn solve_type3(codes: Codes, input: Vec<String>) -> SolveResult {
    let decoded = match input.get(0) {
        Some(s) => decode(&codes, s),
        None => return Err("Нет входных данных"),
    };
 
    // Отсееваем результаты, где повторяются символы
    let decoded = Vec::from_iter(decoded.iter().filter(|i| has_unique_chars(i)));

    // В decoded после фильтра ДОЛЖЕН остаться 1 элемент,
    // его и пытаемся вернуть
    match decoded.get(0) {
        Some(s) => Ok(s.chars().count().to_string()),
        None => return Err(UNABLE_TO_SOLVE_ERR_MSG),
    }
}

// --------------------------------------------------------------------------------------

pub fn solve_type4(codes: Codes, input: Vec<String>) -> SolveResult {
    let decoded = match input.get(0) {
        Some(s) => decode(&codes, s),
        None => return Err("Нет входных данных"),
    };
 
    match decoded.get(0) {
        Some(s) => Ok(s.chars().count().to_string()),
        None => return Err(UNABLE_TO_SOLVE_ERR_MSG),
    }
}

// --------------------------------------------------------------------------------------

pub fn solve_type5(codes: Codes, input: Vec<String>) -> SolveResult {
    let decoded = match input.get(0) {
        Some(s) => decode(&codes, s),
        None => return Err("Нет входных данных"),
    };

    if let None = decoded.get(0) {
        return Err("Не могу решить задачу");
    }

    match get_recurring_chars(&decoded.get(0).unwrap()) {
        Some(chars) => Ok(chars.concat()),
        None => Err("Не могу решить задачу"),
    }
}
