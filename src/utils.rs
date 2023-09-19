use std::fmt::Display;
use std::error::Error;
use std::collections::HashSet;

/// Результат решения задачи.
pub type SolveResult = Result<String, SolveError>;

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

/// Перечисление возможных ошибок при решении задачи.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SolveError {
    /// Задачу нельзя решить.
    UnableToSolve,
    /// Что-то другое.
    Other(String),
}

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            Self::UnableToSolve => "Не могу решить задачу!",
            Self::Other(msg) => msg.as_str(),
        };
        write!(f, "{err_msg}")
    }
}

impl Error for SolveError {}

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
