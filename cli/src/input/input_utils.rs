use std::io::{self, Write};

use crate::errors::CLIError;

/// Выводит `prompt` и ждёт ввод пользователя
pub fn input(prompt: &str) -> Result<String, CLIError> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).map_err(|_| CLIError::ReadingError)?;
    Ok(buff.trim().to_string())
}

/// Спрашивает пользователя о чём-либо.
pub fn ask(question: &str) -> Result<bool, CLIError> {
    loop {
        match input(&format!("{} [да/нет]", question))?.to_lowercase().as_str() {
            "да" => return Ok(true),
            "нет" => return Ok(false),
            _ => println!("Неверный ввод! Попробуёте ещё раз!"),
        }
    }
}

/// Выводит `prompt` и даёт пользователю выбрать один из вариантов
pub fn choose<'t, T>(prompt: &str, options: &[(&str, &'t T)]) -> Result<&'t T, CLIError> {
    println!("{prompt}");

    loop {
        for (index, (key, _)) in options.iter().enumerate() {
            println!("{index}. {key}.");
        }

        match input("> ")?.parse::<usize>() {
            Ok(num) if num <= options.len() => return Ok(&options.get(num).unwrap().1),
            Ok(num) => println!("Варианта {num} не существует!"),
            Err(_) => println!("Ошибка ввода!"),
        }
    }
}

/// Выводит `prompt` и читает строки из `stdin` пока не будет введено end
pub fn input_until_end(prompt: &str) -> Result<Vec<String>, CLIError> {
    println!("{prompt} (в конце введите end)");
    let mut strings = Vec::new();

    loop {
        let buff = input("")?;
        if buff == "end" { break; }
        strings.push(buff);
    }
    Ok(strings)
}

