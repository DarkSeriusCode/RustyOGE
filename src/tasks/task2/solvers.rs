use std::collections::HashSet;
use crate::SolveResult;
use crate::task2::types::Codes;
use crate::task2::core::decode;

fn has_unique_chars(string: &String) -> bool {
    let set: HashSet<char> = HashSet::from_iter(string.chars());
    set.len() == string.chars().count()
}

pub fn solve_type1(codes: Codes, input: Vec<String>) -> SolveResult {
    for string in input {
        let decoded = decode(&codes, &string);
        // Если только одно решение, то и возвращаем его
        if decoded.len() == 1 {
            let s = &decoded[0];
            return Ok(s.to_string());
        }
    }
    Err("Не могу решить задачу!")
}

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
        None => return Err("Не могу решить задачу!"),
    }
}

