use std::error::Error;
use std::fmt::Display;
use std::boxed::Box;

#[derive(Debug)]
pub enum CLIError {
    ReadingError,
    IncorrectInput(Box<dyn Error>),
    InvalidInputData,
    UnknownProblem(u32),
}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CLIError::*;
        let err_msg = match self {
            ReadingError      => "Ошибка чтения".to_string(),
            IncorrectInput(e) => format!("Некорректные входные данные {}!", e.to_string()),
            InvalidInputData  => "Введёные данные не прошли валидацию!".to_string(),
            UnknownProblem(n) => format!("Неизвестная задача под номером {}!", n),
        };

        write!(f, "{}", err_msg)
    }
}

impl Error for CLIError {}
