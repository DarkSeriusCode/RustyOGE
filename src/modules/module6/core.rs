use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use regex::Regex;
use crate::module6::consts::{PROGRAM_INPUT_REGEX, PYTHON_INTERPRETER_CMDS};

/// Конвертирует входные данные из формата `(a, b, ...); (c, d, ...)` в формат
/// `vec![vec![a, b, ...], vec![c, d, ...]]`
pub fn convert_program_input(program_input_str: &str) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    let re = Regex::new(PROGRAM_INPUT_REGEX).unwrap();
    for cap in re.captures_iter(program_input_str) {
        res.push(Vec::from_iter(
            cap[0].split(", ").map(|i| i.trim().to_string())
        ));
    }
    res
}

/// Проверяет наличие Python и возвращает команду для его зауска
pub fn find_python() -> Option<String> {
    for command in PYTHON_INTERPRETER_CMDS {
        let proc_res = Command::new(command)
            .stdout(Stdio::null())
            .arg("--version")
            .spawn();
        if proc_res.is_ok() {
            return Some(command.to_string());
        }
    }
    None
}

/// Запускает Python программу с входными данными и возвращает вывод программы
pub fn run_program(python_cmd: &str, program_path: &Path,
                   program_args: &Vec<String>) -> Result<String, io::Error>
{
    let program_path = program_path.to_str().unwrap();
    let program_args = program_args.join("\n");

    // Запуск программы
    let mut proc = Command::new(python_cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(program_path)
        .spawn()?;

    proc.stdin.as_mut().unwrap().write_all(program_args.as_bytes())?;
    proc.wait()?;

    // Получение вывода
    let mut buffer = String::new();
    proc.stdout.as_mut().unwrap().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

