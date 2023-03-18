use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};

use regex::Regex;

use crate::module6::types::{ProgramInput, PROGRAM_INPUT_REGEX};

// Во входных данных для обозначения минуса и других символов может встретиться несколько 
// Unicode символов поэтому их необходимо заменить на ASCII
const ALT_CHARS: [(&[char], char); 2] = [(&['−', '–', '—'], '-'), (&['\u{2009}'], ' ')];

/// Заменяет Unicode символы на ASCII
fn replace_unicode_chars(string: &str) -> String {
    let mut string = string.to_string();
    for (unicode_ch_list, ch) in ALT_CHARS {
        for unicode_ch in unicode_ch_list {
            string = string.replace(*unicode_ch, &ch.to_string());
        }
    }
    string.to_string()
}

/// Трансформирует строку входных данных из вида (a, b, ...); (c, d, ...) в
/// vec![vec![a, b, ...], vec![c, d, ...]]
pub fn format_program_input(program_input: &str) -> Vec<ProgramInput> {
    let mut res = Vec::new();
    let re = Regex::new(PROGRAM_INPUT_REGEX).unwrap();
    for cap in re.captures_iter(&replace_unicode_chars(program_input)) {
        res.push(Vec::from_iter(cap[0].split(", ").map(|i| i.trim().to_string())));
    }
    res
}

/// Запускает Python программу с входными данными и возвращает вывод программы
pub fn run_program(program_path: &Path, args: &ProgramInput) -> Result<String, io::Error> {
    let program_path = program_path.to_str().unwrap();
    let args = args.join("\n");

    let mut proc = Command::new("python3")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(program_path)
        .spawn()?;
    proc.stdin.as_mut().unwrap().write_all(args.as_bytes())?;
    proc.wait()?;

    // читаем выод из stdout
    let mut buffer = String::new();
    proc.stdout.as_mut().unwrap().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

