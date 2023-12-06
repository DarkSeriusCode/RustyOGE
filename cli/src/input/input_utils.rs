use std::io::{self, Write};
use std::fmt::Display;
use std::str::FromStr;

use colored::Colorize;

use crate::errors::{CLIResult, CLIError};

// ------------------------------------------------------------------------------------------------

/// Выводит `prompt` и ждёт ввод пользователя
pub fn input<T: FromStr>(prompt: &(impl Display + ?Sized)) -> CLIResult<T> {
    print!("{} ", prompt.to_string().underline());
    io::stdout().flush().expect("Возникла критическая ошибка при попытке вевести текст в stdout!");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(|_| CLIError::ReadingError)?;

    T::from_str(buffer.trim())
        .map_err(|_| CLIError::IncorrectInput("Возможно ожидасля другой тип данных".into()))
}

// ------------------------------------------------------------------------------------------------

/// Спрашивает пользователя о чём-либо.
pub fn ask(question: &str) -> CLIResult<bool> {
    choose(question, &[("да", &true), ("нет", &false)]).map(|i| i.to_owned())
}

// ------------------------------------------------------------------------------------------------

/// Выводит `prompt` и даёт пользователю выбрать один из вариантов
pub fn choose<'t, T: ?Sized>(prompt: &str, options: &[(&str, &'t T)]) -> CLIResult<&'t T> {
    loop {
        println!("\n{}", prompt.underline());

        for (index, (key, _)) in options.iter().enumerate() {
            println!("{}. {}", index.to_string().bold(), key);
        }

        match input::<usize>(">") {
            Ok(num) if num < options.len() => return Ok(&options.get(num).unwrap().1),
            Ok(num) => println!("Варианта {} не существует!\n", num.to_string().bold().red()),
            Err(_) => println!("{}", "Введите номер варианта!".bold().red()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Выводит `prompt` и данные строки из `stdin` пока не будет введено end
pub fn input_until_end<T>(prompt: &(impl Display + ?Sized)) -> CLIResult<Vec<T>>
where
    T: FromStr,
{
    println!("{}", format!("{prompt} (в конце введите end)").underline());
    let mut inputed_data = Vec::new();

    loop {
        let buffer: String = input(">")?;
        if buffer == "end" { break; }
        let data_from_str = T::from_str(&buffer)
            .map_err(|_| CLIError::IncorrectInput(buffer.into()))?;
        inputed_data.push(data_from_str);
    }
    Ok(inputed_data)
}
