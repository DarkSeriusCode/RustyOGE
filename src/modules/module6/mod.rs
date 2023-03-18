mod core;
pub mod solvers;
pub mod types;
pub use types::InputData;

use std::boxed::Box;
use std::fmt::Display;
use std::error::Error;

use crate::utils::SolveError;

pub fn solve<F, E>(input: F, type_num: u8) -> Result<Box<dyn Display>, Box<dyn Error>>
where
    F: FnOnce() -> Result<types::InputData, E>,
    E: Error + 'static,
{
    if type_num > solvers::SOLVERS_COUNT {
        return Err(Box::new(SolveError::UnknownProblemType));
    }
    let InputData {file_path, program_input, expected_output } = input()?;

    // Решаем задачу, если ошибка, то возвращаем её
    let res = match type_num {
        1 => solvers::solve_type1(&file_path, &program_input, &expected_output),
        _ => todo!(),
    }?;
    Ok(Box::new(res))
}
