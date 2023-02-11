mod core;
pub mod solvers;
pub mod types;

use crate::utils::Error;
use std::boxed::Box;
use std::fmt::Display;
pub use types::InputData;

pub fn solve<F>(input: F, type_num: u8) -> Result<String, Box<dyn Display>>
where
    F: FnOnce() -> Result<types::InputData, &'static str>,
{
    if type_num > solvers::SOLVERS_COUNT {
        return Err(Box::new(Error::UnknownTaskType));
    }
    let (file_path, program_input) = match input() {
        Ok(InputData {
            file_path,
            program_input,
        }) => (file_path, program_input),
        Err(err) => return Err(Box::new(err)),
    };
    let answer_res = match type_num {
        1 => solvers::solve_type1(&file_path, &program_input),
        _ => todo!(),
    };

    match answer_res {
        Ok(answer) => Ok(answer),
        Err(err) => Err(Box::new(err)),
    }
}
