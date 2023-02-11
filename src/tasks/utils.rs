use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum SolveError {
    UnableToSolve,
    NoInputData,
    UnknownTaskType,
}

impl SolveError {
    pub fn message(&self) -> &str {
        match self {
            Self::UnableToSolve => "Не могу решить задачу!",
            Self::NoInputData => "Нет входных данных!",
            Self::UnknownTaskType => "Неизвестный тип задачи!",
        }
    }
}

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for SolveError {
    fn description(&self) -> &str {
        self.message()
    }
}

