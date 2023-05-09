//! Библиотека для решения задач ОГЭ по информатике 2023 год.
//!
//! ## Структура модуля
//! Для каждого задания существует отдельный модуль, содержащий всё необходимое для решения задачи.
//! Любой модуль обязательно содержит:
//! * Функцию `solve()`. Она решает задачу.
//! * Структуру `InputData`. Это входные данные задачи, а также детали решения.
//! * Структуру `ProblemSpec`. Это те самые детали решения задачи, они указывают как именно нужно
//! нужно решить задачу и что требуется вернуть в качастве ответа.
//!
//! ## Пример
//! Давайте решим [это](https://inf-oge.sdamgia.ru/problem?id=7) задание.
//! ```rust
//! use std::collections::HashMap;
//! use rusty_oge::{SolveResult, utils::Validated};
//! use rusty_oge::module2::{InputData, OutputDataType, ProblemSpec, Codes, solve};
//!
//! let input_data = InputData {
//!     codes: HashMap::from_iter([
//!     ("01", "А"), ("100", "Д"), ("101", "К"), ("10", "Н"), ("111", "О"), ("000", "С")
//!     ].map(|(code, letter)| (code.to_string(), letter.to_string()))
//!     ),
//!     encoded_strings: vec![
//!         "10111101".to_string(),
//!         "1010110".to_string(),
//!         "10111000".to_string()
//!     ],
//!     spec: ProblemSpec {
//!         one_decoding: true,
//!         unique_chars: false,
//!         output_data_type: OutputDataType::DecodedString,
//!     }
//! };
//!
//! // Удостоверимся, что ввели правильные данные
//! assert!(input_data.is_valid());
//!
//! let right_answer = "НОС".to_string();
//!
//! assert_eq!(solve(input_data), Ok(right_answer));
//! ```
//!
//! Само собой это можно записать и короче, но для лучшего понимания мы написали подробнее.
//!

mod modules;

pub use modules::module2;

pub use modules::utils;
pub use utils::{SolveResult, SolveError};

