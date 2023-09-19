/// Модуль 2-го задания
///
/// Модуль для решения второго задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?category_id=7&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=7) задания
/// ```rust
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module2::{InputData, OutputDataType, ProblemSpec, Codes, solve};
///
/// let input_data = InputData {
///     codes: Codes::from_iter([
///     ("01", "А"), ("100", "Д"), ("101", "К"), ("10", "Н"), ("111", "О"), ("000", "С")
///     ].map(|(code, letter)| (code.to_string(), letter.to_string()))
///     ),
///     encoded_strings: vec![
///         "10111101".to_string(),
///         "1010110".to_string(),
///         "10111000".to_string()
///     ],
///     spec: ProblemSpec {
///         one_decoding: true,
///         unique_chars: false,
///         output_data_type: OutputDataType::DecodedString,
///     }
/// };
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "НОС".to_string();
///
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module2;

/// Модуль 6-го задания
///
/// Модуль для решения шестого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?filter=all&category_id=25))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=10458) задания
/// ```rust
/// use std::path::Path;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module6::{InputData, ProblemSpec, solve};
///
/// let input_data = InputData {
///     file_path: Path::new("tests/module6_files/10458.py").to_path_buf(),
///     program_input: "(1, 2); (11, 2); (1, 12); (11, 12); (–11, –12); (–11, 12); (–12, 11); (10, 10); (10, 5).".to_string(),
///     spec: ProblemSpec {
///         expected_output: "YES".to_string(),
///     },
/// };
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "5".to_string();
///
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module6;

/// Модуль 10-го задания
///
/// Модуль для решения десятого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=45&cat_id[]=23&cat_id[]=46&filter=all))
///
/// Пример решения [задания](https://inf-oge.sdamgia.ru/problem?id=16018) где требуется перевести число в другую систему счисления.
/// ```rust
/// use rusty_oge::{num, utils::Validated};
/// use rusty_oge::module10::{InputData, ProblemSpec, solve};
///
/// let input_data = InputData {
///     numbers: vec![num!("1100110", 2)],
///     spec: ProblemSpec::Convert(10),
/// };
///
/// // Проверим корректность данных
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "102".to_string();
///
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module10;
