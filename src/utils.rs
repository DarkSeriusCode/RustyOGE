use std::boxed::Box;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::convert::From;

use unrar::error as unrar_err;

pub mod data_size;

const NORMALIZE_MAP: &[(&[&str], &str)] = &[
    (&["  —"], " -"),
    (&["−", "–", "—"], "-"),
    (&["\u{2009}"], " ")
];

// ------------------------------------------------------------------------------------------------

/// Результат решения задачи.
pub type SolveResult = Result<String, SolveError>;

// ------------------------------------------------------------------------------------------------

/// Какое число требуется найти
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberToFind {
    /// Нужно найти минимальное число
    Min,
    /// Нужно найти максимальное число
    Max,
}

// ------------------------------------------------------------------------------------------------

/// Ошибка, возникающая во время решения задачи.
#[derive(Debug)]
pub struct SolveError(pub Box<dyn Error>);

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot solve the problem! Reason: {}", self.0)
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
            unrar_err::When::Open    => "opening archive",
            unrar_err::When::Read    => "reading archive",
            unrar_err::When::Process => "processing archive",
        };

        SolveError(format!("When {}: {}", when, value).into())
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
pub(crate) fn has_unique_chars(string: &str) -> bool {
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

/// Возвращает вектор со всеми возиожными расстановками `items` на `len` местах контейнера `C`
pub(crate) fn combinations<C, I, U>(len: usize, items: U) -> Vec<C>
where
    C: Default + Extend<I> + Clone,
    I: Clone,
    U: IntoIterator<Item = I> + Clone,
{
    let mut v = vec![];
    if len == 1 {
        for item in items {
            let mut container = C::default();
            container.extend([item]);
            v.push(container);
        }
    } else {
        for combination in combinations::<C, I, U>(len - 1, items.clone()) {
            for item in items.clone() {
                let mut new_combination = combination.clone();
                new_combination.extend([item]);
                v.push(new_combination);
            }
        }
    }
    v
}
