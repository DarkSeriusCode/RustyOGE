use regex::Regex;
use std::path::{Path, PathBuf};

use crate::utils;
use super::consts::PROGRAM_INPUT_REGEX;

// ------------------------------------------------------------------------------------------------

/// Детали выполнения задания. Указывает как имено обработать данные
/// и какие из них выводить.
#[derive(PartialEq, Eq, Clone)]
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
#[derive(PartialEq, Eq, Clone)]
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

impl utils::Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if !self.file_path.exists() {
            return Err(format!("Пути {} не существует!", self.file_path.to_str().unwrap()));
        }
        if !Regex::new(PROGRAM_INPUT_REGEX).unwrap().is_match(&self.program_input) {
            return Err(format!("Неправильный формат входных данных ({})!", self.program_input));
        }
        Ok(())
    }
}
