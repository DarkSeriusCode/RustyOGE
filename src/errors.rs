use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CLIError {
    ReadingError,
    IncorrectInput,
    UnknownTask,
}

impl CLIError {
    pub fn message(&self) -> &str {
        match self {
            Self::ReadingError => "Ошибка чтения!",
            Self::IncorrectInput => "Некорректные входные данные!",
            Self::UnknownTask => "Неизвестная задача!",
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
