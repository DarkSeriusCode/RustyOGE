pub mod solvers;
pub mod types;
mod core;

pub use types::InputData;
use crate::SolveResult;

pub fn solve<F>(input: F, type_num: u8) -> SolveResult
where
    F: FnOnce() -> Result<types::InputData, &'static str>,
{
    if type_num > solvers::SOLVERS_COUNT {
        return Err("Задание 2: Неизвестный тип задачи! (см README.md)")
    }
    let types::InputData{ codes, encoded_strings } = input()?;
    // Решаем задачу, если ошибка, то возвращаем её
    let answer = match type_num {
        1 => solvers::solve_type1(codes, encoded_strings),
        2 => solvers::solve_type2(codes, encoded_strings),
        3 => solvers::solve_type3(codes, encoded_strings),
        4 => solvers::solve_type4(codes, encoded_strings),
        5 => solvers::solve_type5(codes, encoded_strings),
        _ => todo!(),
    }?;
    Ok(answer)
}

