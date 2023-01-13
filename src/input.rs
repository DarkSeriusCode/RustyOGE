use std::io;
use std::collections::HashMap;
use rusty_oge::task2;

/// Выводит `prompt` и читает строки из `stdin` пока не будет введено end
fn input_until_end(prompt: &str) -> Vec<String> {
    println!("{prompt} (в конце введите end)");
    let mut strings = Vec::new();

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Ошибка чтения");
        buff = buff.trim().to_string();
        if buff == "end" { break; }
        strings.push(buff);
    }
    strings
}

// ---------------------------------------------------------------------------

pub fn read_task2_input() -> Result<task2::InputData, &'static str> {
    let mut codes: HashMap<String, String> = HashMap::new();
    // Getting codes
    for s in input_until_end("Введите букву и код через пробел") {
        let pair = Vec::from_iter(s.split_whitespace());
        if pair.len() != 2 {
            return Err("Некорректные входные данные");
        }
        codes.insert(pair[1].to_string(), pair[0].to_string());
    }

    let encoded_strings = input_until_end("Введите строки");

    Ok(task2::InputData::new(codes, encoded_strings))
}

