use super::errors::CLIError;

/// Алиас для типа `Result<T, CLIError>`
pub type CLIResult<T> = Result<T, CLIError>;
