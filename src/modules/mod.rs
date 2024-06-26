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
/// use rusty_oge::module2::{InputData, OutputDataType, Codes, solve};
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
///     one_decoding: true,
///     unique_chars: false,
///     output_data_type: OutputDataType::DecodedString,
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "НОС".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module2;

/// Модуль 3-го задания
///
/// Модуль для решения третьего задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=37&cat_id[]=31&cat_id[]=38&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=10636) задания
/// ```rust
/// use rusty_oge::utils::{NumberToFind, Validated};
/// use rusty_oge::module3::{InputData, solve};
///
/// let input_data = InputData {
///     number_to_find: NumberToFind::Max,
///     digits_in_number: None,
///     expression: "НЕ(Число > 10000) И (Число нечетное).".into(),
///     expression_result: true,
/// };
///
/// // Или используйте метод InputData::new()
///
/// let right_answer = "9999".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
/// Замечу, что числа должны записываться **без пробелов**! Так, например, вместо `10 000` следует
/// написать `10000`
///
///
/// Или вот ещё пример [задания](https://inf-oge.sdamgia.ru/problem?id=18033) где нужно найти максимальное трёхзначное число
/// ```rust
/// use rusty_oge::utils::{NumberToFind, Validated};
/// use rusty_oge::module3::{InputData, solve};
///
/// let input_data = InputData {
///     number_to_find: NumberToFind::Min,
///     digits_in_number: Some(3),
///     expression: "НЕ (Число нечетное) И (Число кратно 3).".into(),
///     expression_result: true,
/// };
///
/// // Или используйте метод InputData::new()
///
/// let right_answer = "102".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module3;

/// Модуль 4-го задания
///
/// Модуль решения четвёртого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=39&cat_id[]=3&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=23) задания
/// ```rust
/// use std::collections::HashMap;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module4::{InputData, PathToFind, solve};
///
/// let input_data = InputData {
///     map: HashMap::from([
///        ("A".into(), vec![("B".into(), 5), ("C".into(), 3)]),
///        ("B".into(), vec![("A".into(), 5), ("C".into(), 1), ("D".into(), 4)]),
///        ("C".into(), vec![("A".into(), 3), ("B".into(), 1), ("D".into(), 6)]),
///        ("D".into(), vec![("B".into(), 4), ("C".into(), 6), ("E".into(), 1)]),
///        ("E".into(), vec![("D".into(), 1)]),
///     ]),
///     way: ("A".into(), "E".into()),
///     include: vec![],
///     path_to_find: PathToFind::Shortest,
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "9".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module4;

/// Модуль 5-го задания
///
/// Модуль решения пятого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=24&cat_id[]=40&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=12854) задания
/// ```rust
/// use std::str::FromStr;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module5::{InputData, ProblemSpec, CommandTable, Command, solve};
///
/// let input_data = InputData {
///     commands: CommandTable::from_iter([
///         ('1', Command::from_str("-b").unwrap()), ('2', Command::from_str("*5").unwrap())
///     ]),
///     begin_num: 2,
///     result_num: 17,
///     spec: ProblemSpec::FindVariableValue("21121".into()),
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "3".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module5;

/// Модуль 6-го задания
///
/// Модуль для решения шестого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?filter=all&category_id=25))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=10458) задания
/// ```rust
/// use std::path::Path;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module6::{InputData, solve};
///
/// let input_data = InputData {
///     file_path: Path::new("tests/module6_files/10458.py").to_path_buf(),
///     program_input: "(1, 2); (11, 2); (1, 12); (11, 12); (–11, –12); (–11, 12); (–12, 11); (10, 10); (10, 5).".to_string(),
///     expected_output: "YES".to_string(),
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "5".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module6;

/// Модуль 7-го задания
///
/// Модуль для решения седьмого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=41&cat_id[]=42&cat_id[]=17&filter=all))
///
/// **ВАЖНО!** Этот модуль решает те седьмые задачки, где требуется составить IP адрес. Остальные
/// седьмые задания настолько просты и наглядны, что нет смысла писать для них алгоритм решения.
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=518) задания
/// ```rust
/// use std::collections::HashMap;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module7::{InputData, solve};
///
/// let input_data = InputData {
///     ip_parts: HashMap::from_iter([
///         ('А', "17".into()), ('Б', ".44".into()), ('В', "4.144".into()), ('Г', "9.13".into())
///     ]),
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "АГВБ".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module7;

/// Модуль 9-го задания
///
/// Модуль для решения девятого задания ОГЭ по информатике. ([каталог заданий](https://inf-oge.sdamgia.ru/test?a=view_many&cat_id[]=44&cat_id[]=22&filter=all))
///
/// Пример решения [этого](https://inf-oge.sdamgia.ru/problem?id=18039) задания
/// ```rust
/// use std::collections::HashMap;
/// use rusty_oge::utils::Validated;
/// use rusty_oge::module9::{InputData, solve};
///
/// let input_data = InputData {
///     map: HashMap::from([
///         ("A".into(), vec![("B".into(), 0), ("G".into(), 0)]),
///         ("B".into(), vec![("D".into(), 0), ("K".into(), 0), ("V".into(), 0)]),
///         ("G".into(), vec![("V".into(), 0), ("K".into(), 0), ("E".into(), 0)]),
///         ("V".into(), vec![("K".into(), 0)]),
///         ("D".into(), vec![("K".into(), 0)]),
///         ("E".into(), vec![("K".into(), 0)]),
///         ("K".into(), vec![]),
///     ]),
///     way: ("A".into(), "K".into()),
///     include: vec![],
///     exclude: vec![],
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "6".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module9;

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
/// let right_answer = "102".to_string();
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
///
/// [Задание](https://inf-oge.sdamgia.ru/problem?id=10323) где требуется найти максимальное/минимальное число в десятичной системе счисления.
/// ```rust
/// use rusty_oge::{num, utils::{NumberToFind, Validated}};
/// use rusty_oge::module10::{InputData, ProblemSpec, solve};
///
/// let input_data = InputData {
///     numbers: vec![num!("23", 16), num!("32", 8), num!("11110", 2)],
///     spec: ProblemSpec::FindNum(NumberToFind::Max),
/// };
/// // Или используйте метод InputData::new()
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
/// use rusty_oge::module12::{InputData, FileInfo, solve};
///
/// let input_data = InputData {
///     archive_path: "tests/module12_files/11322.rar".into(),
///     search_dir: "Task12".into(),
///     extensions: vec!["doc".into()],
///     minimum_file_size: None,
/// };
/// // Или используйте метод InputData::new()
///
/// let right_answer = "3".to_string();
///
/// assert_eq!(solve(input_data), Ok(right_answer));
/// ```
pub mod module12;
