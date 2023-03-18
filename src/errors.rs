use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CLIError {
    ReadingError,
    IncorrectInput,
    UnknownProblem,
}

impl CLIError {
    pub fn message(&self) -> &str {
        match self {
            Self::ReadingError => "Ошибка чтения!",
            Self::IncorrectInput => "Некорректные входные данные!",
            Self::UnknownProblem => "Неизвестная задача!",
        }
    }
}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for CLIError {
    fn description(&self) -> &str {
        self.message()
    }
}
