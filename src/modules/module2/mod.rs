pub mod solvers;
pub mod types;
mod core;

pub use types::InputData;
use crate::utils::SolveError;

use std::boxed::Box;
use std::fmt::Display;
use std::error::Error;

pub fn solve<F, E>(input: F, type_num: u8) -> Result<Box<dyn Display>, Box<dyn Error>>
where
    F: FnOnce() -> Result<types::InputData, E>,
    E: Error + 'static,
{
    if type_num > solvers::SOLVERS_COUNT {
        return Err(Box::new(SolveError::UnknownProblemType));
    }
    let types::InputData{ codes, encoded_strings } = input()?;
    // Решаем задачу, если ошибка, то возвращаем её
    let res = match type_num {
        1 => solvers::solve_type1(codes, encoded_strings),
        2 => solvers::solve_type2(codes, encoded_strings),
        3 => solvers::solve_type3(codes, encoded_strings),
        4 => solvers::solve_type4(codes, encoded_strings),
        5 => solvers::solve_type5(codes, encoded_strings),
        _ => todo!(),
    }?;
    Ok(Box::new(res))
}

