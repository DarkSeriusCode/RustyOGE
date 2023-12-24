/// Модуль 1-го задания
///
/// Модуль для решения первого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=21&cat_id[]=33&cat_id[]=34&cat_id[]=35&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=10313) задания
/// ```rust
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module1::{InputData, InputText, ProblemSpec, solve};
///
/// let input_data = InputData {
///     bits_in_char: 8,
///     text: InputText::ConcreteText("Обь, Лена, Волга, Москва, Макензи, Амазонка  — реки".into()),
///     spec: ProblemSpec::FindWord(8),
/// };
/// // Или используйте метод InputData::new()
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "Москва".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
///
/// Или вот ещё пример [такого](https://inf-oge.sdamgia.ru/problem?id=18255) задания.
/// ```rust
/// use rusty_oge::utils::{Validated, data_size::DataSizeUnit};
/// use rusty_oge::module1::{InputData, InputText, ProblemSpec, solve};
///
/// let input_data = InputData {
///     bits_in_char: 16,
///     text: InputText::TextInfo {
///         pages: 10,
///         lines: 32,
///         chars: 48,
///     },
///     spec: ProblemSpec::CalcTextSize(DataSizeUnit::Kb),
/// };
/// // Или используйте метод InputData::new()
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "30".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module1;

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
/// // Или используйте метод InputData::new()
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "НОС".to_string();
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
/// // Или используйте метод InputData::new()
///
/// // Удостоверимся, что ввели правильные данные
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "5".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module6;

/// Модуль 10-го задания
///
/// Модуль для решения десятого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=45&cat_id[]=23&cat_id[]=46&filter=all))
///
/// ## Пример решения разных заданий.
///
/// [Задание](https://inf-oge.sdamgia.ru/problem?id=16018) где требуется перевести число в другую систему счисления.
/// ```rust
/// use rusty_oge::{num, utils::Validated};
/// use rusty_oge::module10::{InputData, ProblemSpec, solve};
///
/// let input_data = InputData {
///     numbers: vec![num!("1100110", 2)],
///     spec: ProblemSpec::Convert(10),
/// };
/// // Или используйте метод InputData::new()
///
/// // Проверим корректность данных
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "102".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
///
/// [Задание](https://inf-oge.sdamgia.ru/problem?id=10323) где требуется найти максимальное/минимальное число в десятичной системе счисления.
/// ```rust
/// use rusty_oge::{num, utils::Validated};
/// use rusty_oge::module10::{InputData, ProblemSpec, NumberToFind, solve};
///
/// let input_data = InputData {
///     numbers: vec![num!("23", 16), num!("32", 8), num!("11110", 2)],
///     spec: ProblemSpec::FindNum(NumberToFind::Max),
/// };
/// // Или используйте метод InputData::new()
///
/// // Проверим корректность данных
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "35".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
///
/// см. [`ProblemSpec`](crate::module10::ProblemSpec)
pub mod module10;

/// Модуль 12-го задания
///
/// Модуль для решения десятого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=28&cat_id[]=47&filter=all))
///
/// Пример решения [задания](https://inf-oge.sdamgia.ru/problem?id=11322)
/// ```rust
/// use rusty_oge::utils::{Validated, data_size::DataSize};
/// use rusty_oge::module12::{InputData, ProblemSpec, FileInfo, solve};
///
/// let input_data = InputData {
///     archive_path: "tests/module12_files/11322.rar".into(),
///     search_dir: "Task12".into(),
///     spec: ProblemSpec::WithExtencions(vec!["doc".into()]),
/// };
/// // Или используйте метод InputData::new()
///
/// // Проверим корректность данных
/// assert!(input_data.valid().is_ok());
///
/// let right_answer = "3".to_string();
///
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module12;
