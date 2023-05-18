use std::io::{self, Write};
use std::collections::HashMap;
use rusty_oge::module2;
use rusty_oge::utils::Validated;
use crate::errors::CLIError;

/// Выводит `prompt` и ждёт ввод пользователя
fn input(prompt: &str) -> Result<String, CLIError> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    if let Err(_) = io::stdin().read_line(&mut buff) {
        return Err(CLIError::ReadingError);
    }
    Ok(buff.trim().to_string())
}

/// Спрашивает пользователя о чём-либо.
fn ask(question: &str) -> Result<bool, CLIError> {
    let mut buff = String::new();

    loop {
        print!("{question} [Да/Нет] ");
        io::stdout().flush().unwrap();

        if let Err(_) = io::stdin().read_line(&mut buff) {
            return Err(CLIError::ReadingError);
        }
        match buff.to_lowercase().trim() {
            "да" => return Ok(true),
            "нет" => return Ok(false),
            _ => println!("Неверный ввод! Попробуёте ещё раз!"),
        }
        buff.clear();
    }
}

/// Даёт пользователю выбрать один из вариантов
fn choose<'t, T>(prompt: &str, answers: &[(&str, &'t T)]) -> Result<&'t T, CLIError> {
    println!("{prompt}");
    let mut buff = String::new();

    loop {
        for (index, (key, _)) in answers.iter().enumerate() {
            println!("{index}. {key}.");
        }
        print!("> ");
        io::stdout().flush().unwrap();

        if let Err(_) = io::stdin().read_line(&mut buff) {
            return Err(CLIError::ReadingError);
        }
        match buff.trim().parse::<usize>() {
            Ok(num) if num <= answers.len() => return Ok(&answers.get(num).unwrap().1),
            Ok(num) => println!("Варианта {num} не существует!"),
            Err(_) => println!("Ошибка ввода!"),
        }
        buff.clear();
    }
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

    // Если нужно только найти строку с одной расшифровкой, то не спрашиваем остальное
    let only_decode = ask("Нужно найти строку, имеющую только одну расшифровку?")?;
    let mut only_unique_chars = false;
    let data_format = choose("Какие данные вывести?", &[
                        ("Декодированную строку", &module2::OutputDataType::DecodedString),
                        ("Длину строки",          &module2::OutputDataType::Length),
                        ("Повторяющиеся символы", &module2::OutputDataType::RepeatingChars)
    ])?;
    if !only_decode {
        only_unique_chars = ask("В строке символы не должны повторяться?")?;
    }

    let spec = module2::ProblemSpec::new(only_decode, only_unique_chars, *data_format);
    let input_data = module2::InputData::new(codes, encoded_strings, spec);
    if !input_data.is_valid() {
        return Err(CLIError::IncorrectInput);
    }

    Ok(input_data)
}

