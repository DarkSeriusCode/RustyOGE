use std::path::PathBuf;
use std::io::{self, Write};
use std::collections::HashMap;
use rusty_oge::{module2, module6};
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

pub fn read_module2_input() -> Result<module2::InputData, CLIError> {
    let mut codes: HashMap<String, String> = HashMap::new();

    for s in input_until_end("Введите букву и код через пробел")? {
        let pair = Vec::from_iter(s.split_whitespace());
        if pair.len() != 2 {
            return Err(CLIError::IncorrectInput);
        }
        codes.insert(pair[1].to_string(), pair[0].to_string());
    }

    let encoded_strings = input_until_end("Введите строки")?;

    Ok(module2::InputData::new(codes, encoded_strings))
}

// --------------------------------------------------------------------------------------

pub fn read_module6_input() -> Result<module6::InputData, CLIError> {
    let path_str = input("Введите путь до файла с программой: ")?;
    let input_string = input("Введите входные данные: ")?;
    let expected_output = input("Что должна вывести программа (пиши с учётом регистра): ")?;

    let mut path = PathBuf::new();
    path.push(&path_str);

    let input_data = module6::InputData::new(
        path,
        &input_string,
        &expected_output,
    );
    if !input_data.is_valid() {
        return Err(CLIError::IncorrectInput);
    }
    Ok(input_data)
}
