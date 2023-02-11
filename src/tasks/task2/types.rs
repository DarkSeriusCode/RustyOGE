pub use std::collections::HashMap;
use crate::utils::SolveError;

/// Алиас для сокращённой записи таблиц кодов
pub type Codes = HashMap<String, String>;

pub struct InputData {
    /// codes - таблица кодов (код - буква)
    pub codes: Codes,
    /// encoded_strings - строки для декодирования
    pub encoded_strings: Vec<String>,
}

impl InputData {
    pub fn new(codes: Codes, encoded_strings: Vec<String>) -> Self {
        Self { codes, encoded_strings }
    }
}

// --------------------------------------------------------------------------------------

pub type SolveResult = Result<String, SolveError>;
