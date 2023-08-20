use std::path::PathBuf;

use rusty_oge::module6;
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use super::input_utils::*;

pub fn get_input() -> Result<module6::InputData, CLIError> {
    let path = input("Введите путь до файла с программой: ")?;
    let input_string = input("Введите данные для программы: ")?;
    let expected_output = input("Что должна вывести программа: ")?;

    let mut program_path_buf = PathBuf::new();
    program_path_buf.push(&path);

    let spec = module6::ProblemSpec::new(expected_output);
    let input_data = module6::InputData::new(&program_path_buf, &input_string, spec);

    if !input_data.is_valid() {
        return Err(CLIError::InvalidInputData);
    }

    Ok(input_data)
}
