use std::boxed::Box;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::convert::From;

use unrar::error as unrar_err;

pub mod data_size;

const NORMALIZE_MAP: &[(&[&'static str], &'static str)] = &[
    (&["  —"], " -"),
    (&["−", "–", "—"], "-"),
    (&["\u{2009}"], " ")
];

// ------------------------------------------------------------------------------------------------

/// Результат решения задачи.
pub type SolveResult = Result<String, SolveError>;

// ------------------------------------------------------------------------------------------------

/// Ошибка, возникающая во время решения задачи.
#[derive(Debug)]
pub struct SolveError(pub Box<dyn Error>);

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Невозможно решить задачу! Причина: {}", self.0)
    }
}

impl PartialEq for SolveError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<unrar_err::UnrarError> for SolveError {
    fn from(value: unrar_err::UnrarError) -> Self {
        let when = match value.when {
            unrar_err::When::Open    => "открытия архива",
            unrar_err::When::Read    => "чтения архива",
            unrar_err::When::Process => "обработки архива",
        };

        SolveError(format!("Во время {} произошла ошибка: {}", when, value.to_string()).into())
    }
}

impl Error for SolveError {}
impl Eq for SolveError {}

// ------------------------------------------------------------------------------------------------

/// Данные, которые могут быть проверены на валидность.
/// Этот `trait` **должна** реализовывать каждая структура входных данных.
pub trait Validated {
    /// Проверяет корректность данных.
    /// Возврашает `Ok(())` если валидация прошла успешно, иначе Err(String), с причиной
    /// непройденной валидации.
    fn valid(&self) -> Result<(), String>;
}

// ------------------------------------------------------------------------------------------------

/// Во входных данных иногда может встретиться несколько Unicode символов для обозначения одного и
/// того же знака. Эта функция заменяет их на ASCII
pub fn normalize_text(text: &str) -> String {
    let mut text = text.to_string();
    for (patterns, replacer) in NORMALIZE_MAP {
        patterns.iter().for_each(|p| text = text.replace(p, replacer));
    }
    text
}

// ------------------------------------------------------------------------------------------------

/// Возвращает `true`, если ни один символ в строке не повторяется, иначе `false`
pub(crate) fn has_unique_chars(string: &String) -> bool {
    let set: HashSet<char> = HashSet::from_iter(string.chars());
    set.len() == string.chars().count()
}

/// Возвращает отсортированный `Vec`, содержащий все повторяющиеся символы в `string`
pub(crate) fn get_repeating_chars(string: &str) -> Vec<char> {
    let mut string_set: HashSet<char> = HashSet::from_iter(string.chars());
    let repeating_chars: HashSet<char> = HashSet::from_iter(string.chars()
                                                           .filter(|ch| !string_set.remove(ch)));

    let mut v = Vec::from_iter(repeating_chars.iter().map(|c| c.to_owned()));
    v.sort();
    v
}
