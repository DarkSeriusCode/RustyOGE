use std::path::PathBuf;

use regex::Regex;

use crate::utils::SolveError;

pub const PROGRAM_INPUT_REGEX: &str = r"([^\(]?\d+,?\s?)+";

// --------------------------------------------------------------------------------------

pub type ProgramInput = Vec<String>;

// --------------------------------------------------------------------------------------

pub struct InputData {
    pub file_path: PathBuf,       // Путь до файла с программой 
    pub program_input: String,    // Входные данные для программы
    pub expected_output: String,  // Ожидаемый вывод программы
}

impl InputData {
    pub fn new(file_path: PathBuf, program_input: &str, expected_out: &str) -> Self {
        Self {
            file_path: file_path.clone(),
            program_input: program_input.to_string(),
            expected_output: expected_out.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.file_path.exists() {
            return false;
        }

        if !Regex::new(PROGRAM_INPUT_REGEX).unwrap().is_match(&self.program_input) {
            return false;
        }

        true
    }
}

// --------------------------------------------------------------------------------------

pub type SolveResult = Result<String, SolveError>;

