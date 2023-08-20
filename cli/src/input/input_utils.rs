use std::io::{self, Write};
use std::fmt::Display;
use std::str::FromStr;

use crate::errors::CLIError;
use crate::utils::CLIResult;

/// Выводит `prompt` и ждёт ввод пользователя
pub fn input<T: FromStr>(prompt: &(impl Display + ?Sized)) -> CLIResult<T> {
    print!("{prompt}");
    io::stdout().flush().expect("An error has occurred when printing to stdout!");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(|_| CLIError::ReadingError)?;

    T::from_str(buffer.trim())
        .map_err(|_| CLIError::IncorrectInput("Возможно ожидасля другой тип данных".into()))
}

/// Спрашивает пользователя о чём-либо.
pub fn ask(question: &str) -> CLIResult<bool> {
    loop {
        match input::<String>(&format!("{} [да/нет]", question))?.to_lowercase().as_str() {
            "да" => return Ok(true),
            "нет" => return Ok(false),
            _ => println!("Неверный ввод! Попробуёте ещё раз!\n"),
        }
    }
}

/// Выводит `prompt` и даёт пользователю выбрать один из вариантов
pub fn choose<'t, T: ?Sized>(prompt: &str, options: &[(&str, &'t T)]) -> CLIResult<&'t T> {
    println!("{prompt}");

    loop {
        for (index, (key, _)) in options.iter().enumerate() {
            println!("{index}. {key}.");
        }

        match input::<usize>("> ")? {
            num if num <= options.len() => return Ok(&options.get(num).unwrap().1),
            num => println!("Варианта {num} не существует!\n"),
        }
    }
}

/// Выводит `prompt` и данные строки из `stdin` пока не будет введено end
pub fn input_until_end<T>(prompt: &(impl Display + ?Sized)) -> CLIResult<Vec<T>>
where
    T: FromStr,
{
    println!("{prompt} (в конце введите end)");
    let mut inputed_data = Vec::new();

    loop {
        let buffer: String = input("")?;
        if buffer == "end" { break; }
        let data_from_str = T::from_str(&buffer)
            .map_err(|_| CLIError::IncorrectInput(buffer.into()))?;
        inputed_data.push(data_from_str);
    }
    Ok(inputed_data)
}

