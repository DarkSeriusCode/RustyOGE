use std::str::FromStr;

use super::errors::CLIError;

/// Алиас для типа `Result<T, CLIError>`
pub type CLIResult<T> = Result<T, CLIError>;

// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePairError;

/// Представляет пару значений.
pub struct Pair<F, S>
where
    F: FromStr,
    S: FromStr,
{
    first: F,
    second: S,
}

impl<F, S> Pair<F, S>
where
    F: FromStr,
    S: FromStr,
{
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &F {
        &self.first
    }

    pub fn second(&self) -> &S {
        &self.second
    }
}

impl<F, S> FromStr for Pair<F, S>
where
    F: FromStr,
    S: FromStr,
{
    type Err = ParsePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = Vec::from_iter(s.split_whitespace());

        if parts.len() != 2 {
            return Err(ParsePairError);
        }

        let first_from_str = F::from_str(parts[0]).map_err(|_| ParsePairError)?;
        let second_from_str = S::from_str(parts[1]).map_err(|_| ParsePairError)?;

        Ok(Pair::new(first_from_str, second_from_str))
    }
}
