use std::io::{self, Write};
use std::collections::HashMap;
use rusty_oge::task2;
use crate::errors::CLIError;

// Выводит `prompt` и ждёт ввод пользователя
fn input(prompt: &str) -> Result<String, CLIError> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    if let Err(_) = io::stdin().read_line(&mut buff) {
        return Err(CLIError::ReadingError);
    }
    Ok(buff.trim().to_string())
}

/// Выводит `prompt` и читает строки из `stdin` пока не будет введено end
fn input_until_end(prompt: &str) -> Result<Vec<String>, CLIError> {
    println!("{prompt} (в конце введите end)");
    let mut strings = Vec::new();

    loop {
        let buff = input("")?;
        if buff == "end" { break; }
        strings.push(buff);
    }
    Ok(strings)
}

// --------------------------------------------------------------------------------------

pub fn read_task2_input() -> Result<task2::InputData, CLIError> {
    let mut codes: HashMap<String, String> = HashMap::new();

    for s in input_until_end("Введите букву и код через пробел")? {
        let pair = Vec::from_iter(s.split_whitespace());
        if pair.len() != 2 {
            return Err(CLIError::IncorrectInput);
        }
        codes.insert(pair[1].to_string(), pair[0].to_string());
    }

    let encoded_strings = input_until_end("Введите строки")?;

    Ok(task2::InputData::new(codes, encoded_strings))
}

// --------------------------------------------------------------------------------------


