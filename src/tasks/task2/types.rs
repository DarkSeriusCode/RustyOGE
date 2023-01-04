pub use std::collections::HashMap;

/// Алиас для сокращённой записи таблиц кодов
pub type Codes = HashMap<String, String>;

/// Представляет входные данные задачи
/// codes - таблица кодов (код - буква)
/// encoded_strings - строки для декодирования
pub struct InputData {
    pub codes: Codes,
    pub encoded_strings: Vec<String>,
}

