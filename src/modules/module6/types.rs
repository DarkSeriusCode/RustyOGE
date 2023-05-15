use regex::Regex;
use std::path::{Path, PathBuf};

use crate::utils;
use crate::module6::consts::PROGRAM_INPUT_REGEX;

// ------------------------------------------------------------------------------------------------

/// Детали выполнения задания. Указывает как имено обработать данные
/// и какие из них выводить.
#[derive(PartialEq, Eq, Clone)]
pub struct ProblemSpec {
    /// Ожидаемый вывод программы
    pub excepted_output: String,
}

impl ProblemSpec {
    pub fn new(excepted_output: String) -> Self {
        Self { excepted_output }
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
    fn is_valid(&self) -> bool {
        if !self.file_path.exists() {
            return false;
        }
        if !Regex::new(PROGRAM_INPUT_REGEX).unwrap().is_match(&self.program_input) {
            return false;
        }
        true
    }
}

