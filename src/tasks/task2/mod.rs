pub mod solvers;
pub mod types;
mod core;

pub use types::InputData;
use crate::SolveResult;

pub fn solve<F>(input: F, type_num: u8) -> SolveResult
where
    F: FnOnce() -> Result<types::InputData, &'static str>,
{
    let input_data = input()?;
    let (codes, strings) = (input_data.codes, input_data.encoded_strings);
    // Решаем задачу, если ошибка, то возвращаем её
    let answer = match type_num {
        1 => solvers::solve_type1(codes, strings),
        2 => solvers::solve_type2(codes, strings),
        _ => Err("Задание 2: неизвестный тип задачи! (см. README.md)"),
    }?;
    Ok(answer)
}

