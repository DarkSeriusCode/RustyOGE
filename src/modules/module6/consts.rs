/// Unicode символы для замены на соответствующие ASCII символы
pub const ALT_CHARS: [(&[char], char); 2] = [(&['−', '–', '—'], '-'), (&['\u{2009}'], ' ')];

/// Регулярное выражение для выделения аргументов Python программы из строки входных данных
pub const PROGRAM_INPUT_REGEX: &str = r"([^\(]?\d+,?\s?)+";

/// Возможные команды для вызова интерпретатора Python
pub const PYTHON_INTERPRETER_CMDS: [&str; 2] = ["python", "python3"];

