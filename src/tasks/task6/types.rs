use std::path::PathBuf;

use regex::Regex;

pub const PROGRAM_INPUT_REGEX: &str = r"\((.?\d+), (.?\d+)\)[;\.]";

// --------------------------------------------------------------------------------------

pub struct ProgramInput(pub String, pub String);

// --------------------------------------------------------------------------------------

/// file_path - Путь до файла с программой
/// program_input - Входные данные для программы
pub struct InputData {
    pub file_path: PathBuf,
    pub program_input: String,
}

impl InputData {
    pub fn new(file_path: PathBuf, program_input: String) -> Self {
        Self {
            file_path: file_path.clone(),
            program_input,
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

