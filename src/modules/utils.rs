use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum SolveError {
    UnableToSolve,
    NoInputData,
    UnknownProblemType,
    Other(String),
}

impl SolveError {
    pub fn message(&self) -> &str {
        match self {
            Self::UnableToSolve => "Не могу решить задачу!",
            Self::NoInputData => "Нет входных данных!",
            Self::UnknownProblemType => "Неизвестный тип задачи!",
            Self::Other(msg) => msg.as_str(),
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

