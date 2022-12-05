// Решение второго задания ОГЭ
use std::collections::HashMap;
use std::io;

pub fn solve() -> Result<(), &'static str> {
    let codes = get_codes()?;
    let input = get_input_data();

    let mut buffer = String::new();
    for ch in input.chars() {
        buffer.push(ch);
        if let Some(letter) = codes.get(&buffer) {
            buffer.clear();
            print!("{letter}");
        }
    }
    println!();
    Ok(())
}

// Gets input data form stdin
fn get_input_data() -> String {
    let mut buffer = String::new();
    println!("Введи сообщение: ");
    io::stdin().read_line(&mut buffer).expect("Reading error");
    buffer
}

// Gets letters and their codes from stdin and returns as HashMap
fn get_codes() -> Result<HashMap<String, String>, &'static str> {
    let mut map = HashMap::new();

    // TODO: Refactor this
    for _ in 0..7 {
        let mut buffer = String::new();

        println!("Введи букву и код через пробел: ");
        io::stdin().read_line(&mut buffer).expect("Reading failed!");
        
        let input = Vec::from_iter(buffer.split_whitespace());
        if input.len() != 2 {
            return Err("Ошибка формата!");
        } 
        // input[0] - letter, input[1] - code
        map.insert(input[1].to_string(), input[0].to_string());
    }
    Ok(map)
}
