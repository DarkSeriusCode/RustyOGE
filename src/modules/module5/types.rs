use std::{
    str::FromStr,
    fmt::Display,
    error::Error,
    collections::HashMap,
    ops::{Add, Mul, Sub, Div},
};

use crate::utils::Validated;

/// Алиас для `HashMap<char, Command>`
pub type CommandTable = HashMap<char, Command>;

// ------------------------------------------------------------------------------------------------

/// Детали решения задания. Указывает, что нужно сделать в задаче
#[derive(Debug, Clone)]
pub enum ProblemSpec {
    /// Находит значение переменной в программе `String`
    FindVariableValue(String),
    /// Составляет алгоритм из команд, содержащий не более `usize` команд
    MakeAlgorithm(usize),
}

// ------------------------------------------------------------------------------------------------

/// Ошибка парсинга `Command`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCommandError(pub String);

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseCommandError {}

// ------------------------------------------------------------------------------------------------

/// Команда исполнителя
///
/// Строковый формат (используется в `from_str`): `<operator><value>`
///
/// Где `<operator>`:
/// - `+` - прибавить `<value>` к числу
/// - `-` - вычесть `<value>` из числа
/// - `*` - умножить на `<value>`
/// - `/` - разделить на `<value>`
/// - `^` - Возвести в степень `<value>`
///
/// `<value>` может быть как числом, так и просто символом, в этом случае символ расценивается как
/// переменная
#[derive(Debug, Clone)]
pub struct Command {
    /// Функция, вызываемая при использовании `execute`
    action: fn(f32, f32) -> f32,
    /// Если `Some()`, то при вызове `execute` используется как второй аргумент `action`, а второй
    /// аргумент, переданный в `execute` будет проигнорирован. Если `None`, то оба аргумента
    /// `execute` передаются в `action`
    action_value: Option<f32>,
}

impl Command {
    /// Создаёт новую команду для исполнителя.
    /// `action` - функция, вызываемая в `execute` и принимающая первым параметром число на экране,
    /// а вторым `action_value`
    /// `action_value` - если Some(n), то в кацестве второго аргумента `action` передаётся n, если
    /// None, то передаётся значение переменной
    pub fn new(action: fn(f32, f32) -> f32, action_value: Option<f32>) -> Self {
        Self { action, action_value, }
    }

    /// Выполняет команду исполнителя.
    pub fn execute(&self, acc: f32, num: f32) -> f32 {
        (self.action)(acc, self.action_value.unwrap_or(num))
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(' ', "");

        let action = match s.chars().next().unwrap() {
            '+' => Add::add,
            '-' => Sub::sub,
            '*' => Mul::mul,
            '/' => Div::div,
            '^' => |num: f32, e: f32| num.powf(e),
            other => return Err(ParseCommandError(format!("Unknown action! {}\
                                               Possible actions are +, -, *, /, ^", other))),
        };

        let action_value = match s[1..].parse::<f32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        };

        Ok(Self::new(action, action_value))
    }
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Команды исполнителя
    pub commands: CommandTable,
    /// Начальное число
    pub begin_num: i32,
    /// Число, которое должно получиться
    pub result_num: i32,
    /// Детали решения задачи
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new(commands: CommandTable, begin_num: i32, result_num: i32, spec: ProblemSpec) -> Self {
        Self {
            commands,
            begin_num,
            result_num,
            spec,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if let ProblemSpec::FindVariableValue(ref program) = self.spec {
            for cmd_name in program.chars() {
                if self.commands.get(&cmd_name).is_none() {
                    return Err(format!("Unknown command {} in \"{}\"!", cmd_name, program));
                }
            }
        }
        Ok(())
    }
}
