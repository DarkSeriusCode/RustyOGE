use regex::Regex;
use std::path::{Path, PathBuf};

use crate::utils::Validated;
use super::consts::PROGRAM_INPUT_REGEX;

/// Детали решения задачи. Хранит дополнительную информацию о задаче.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProblemSpec {
    /// Ожидаемый вывод программы
    pub expected_output: String,
}

impl ProblemSpec {
    pub fn new(expected_output: String) -> Self {
        Self { expected_output }
    }
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputData {
    /// Путь до файла с программой
    pub file_path: PathBuf,
    /// Строка входных данных для программы
    pub program_input: String,
    /// Детали выполнения
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new(file_path: &Path, program_input: &str, spec: ProblemSpec) -> Self {
        Self {
            file_path: file_path.to_path_buf(),
            program_input: program_input.to_string(),
            spec,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if !self.file_path.exists() {
            return Err(format!("File {} doesn't exist", self.file_path.to_str().unwrap()));
        }
        if !Regex::new(PROGRAM_INPUT_REGEX).unwrap().is_match(&self.program_input) {
            return Err(format!("Invalid format of given string ({})", self.program_input));
        }
        Ok(())
    }
}
