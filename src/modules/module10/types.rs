use crate::module10::Number;
use crate::utils::Validated;

/// Какое число требуется найти
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NumberToFind {
    /// Нужно найти минимальное число
    Min,
    /// Нужно найти максимальное число
    Max,
}

// ------------------------------------------------------------------------------------------------

/// Детали выполнения задания. Указывает, что требуется сделать в задаче.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ProblemSpec {
    /// В задаче требуется найти наименьшее/наибольшее число в десятичной системе счисления
    FindNum(NumberToFind),
    /// В задаче требуется перевести число в другую систему счисления
    Convert(u32),
    /// В задаче требуется найти число, сумма цифр которого наибольшая/наименьшая в заданной
    /// системе счисления
    FindDigitsSum(u32, NumberToFind),
    /// В задаче требуется найти число с наименьшим/наибольшим числом единиц в двоичной системе
    /// счисления
    FindOnesCount(NumberToFind),
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(PartialEq, Eq, Clone)]
pub struct InputData {
    /// Данные в задаче числа
    pub numbers: Vec<Number>,
    /// Детали выполнения
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new(numbers: Vec<Number>, spec: ProblemSpec) -> Self {
        Self { numbers, spec }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if self.numbers.len() == 0 {
            return Err("Вы должны ввести хотя бы одно число!".into())
        }
        Ok(())
    }
}
