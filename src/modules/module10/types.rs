use crate::module10::Number;

use crate::utils;

// ------------------------------------------------------------------------------------------------

/// Какое число требуется найти
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NumberToFind {
    Min,
    Max,
}

/// Детали выполнения задания. Указывает как имено обработать данные
/// и какие из них выводить.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ProblemSpec {
    FindNum(NumberToFind),
    Convert(u32),
    FindDigitsSum(u32, NumberToFind),
    FindOnesCount(NumberToFind),
}

/// Входные данные задачи.
#[derive(PartialEq, Eq, Clone)]
pub struct InputData {
    pub numbers: Vec<Number>,
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new(numbers: Vec<Number>, spec: ProblemSpec) -> Self {
        Self { numbers, spec }
    }
}

impl utils::Validated for InputData {
    fn is_valid(&self) -> bool {
        self.numbers.len() != 0
    }
}

