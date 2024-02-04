use std::collections::HashMap;

use crate::utils::{self, Validated};

/// Алиас для сокращённой записи таблиц кодов. Ключ - код, значение - символ
pub type Codes = HashMap<String, String>;

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputData {
    /// Таблица кодов
    pub codes: Codes,
    /// Закодированные строки
    pub encoded_strings: Vec<String>,
    /// `true`, если нужно **только** найти строку с одной расшифровкой.
    pub one_decoding: bool,
    /// Фильтрует расшифровки строки, оставляя только те, где символы не повторяются.
    pub unique_chars: bool,
    /// Тип возвращаемых данных
    pub output_data_type: OutputDataType,
}

impl InputData {
    pub fn new(codes: Codes, encoded_strings: Vec<String>,
               one_decoding: bool, unique_chars: bool, output_data_type: OutputDataType) -> Self {
        Self { codes, encoded_strings, one_decoding, unique_chars, output_data_type, }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if self.encoded_strings.len() == 0 {
            return Err("You must specify at least one string".into());
        }
        if self.unique_chars && self.encoded_strings.len() != 1 {
            return Err("You must specify only one string to find a decoding with \
                        unique characters".into());
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

/// Тип выходных данных
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputDataType {
    /// Возвращает декодированную строку
    DecodedString,
    /// Возвращает длину декодированной строки
    Length,
    /// Возвращает повторяющиеся в декодированной строке символы
    RepeatingChars,
}

impl OutputDataType {
    /// Форматирует строку выходных данных в соответствии с `Self`
    pub fn format(&self, output_data: &str) -> String {
        match self {
            Self::DecodedString  => output_data.to_string(),
            Self::Length         => output_data.chars().count().to_string(),
            Self::RepeatingChars => utils::get_repeating_chars(output_data).iter().collect(),
        }
    }
}
