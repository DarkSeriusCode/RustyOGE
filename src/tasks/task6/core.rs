use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::task6::types::{ProgramInput, PROGRAM_INPUT_REGEX};
use crate::utils::Error;
use crate::SolveResult;
use regex::Regex;

fn create_temp_file(name: &Path, content: &str) -> Result<File, io::Error> {
    let mut path = PathBuf::new();
    path.push(env::temp_dir());
    path.push(name);

    let mut temp_file = File::create(&path)?;
    temp_file.write_all(content.as_bytes())?;

    Ok(File::open(&path)?)
}

/// Форматирует строку входных данных из вида (a, b); (c, d) в
/// Vec::from([("a", "b"), ("c", "d")])
pub fn format_program_input(program_input: &String) -> Vec<ProgramInput> {
    let mut res = Vec::new();
    let re = Regex::new(PROGRAM_INPUT_REGEX).unwrap();
    for cap in re.captures_iter(&program_input.replace("–", "-")) {
        res.push(ProgramInput(cap[1].to_string(), cap[2].to_string()));
    }
    res
}

/// Запускает Python программу с входными данными и возвращает вывод программы
pub fn run_program(program_path: &Path, args: &ProgramInput) -> SolveResult {
    let tmp_fname = format!("temp-{:?}", program_path.file_name().unwrap());
    let tmp_file_res = create_temp_file(
        &Path::new(&tmp_fname),
        &format!("{}\n{}\n", &args.0, &args.1),
    );
    let temp_file = match tmp_file_res {
        Ok(file) => file,
        Err(_) => return Err(Error::UnableToCreateTmpFile),
    };

    // Запуск программы
    let output = Command::new("python3")
        .arg(program_path.to_str().unwrap())
        .stdin(temp_file)
        .output()
        .expect("Не могу запустить программу");

    let res = String::from_utf8(output.stdout).unwrap();
    Ok(res.trim().to_string())
}
