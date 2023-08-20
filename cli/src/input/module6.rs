use std::path::PathBuf;

use rusty_oge::module6;
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use super::input_utils::*;

pub fn get_input() -> Result<module6::InputData, CLIError> {
    let path: PathBuf           = input("Введите путь до файла с программой: ")?;
    let input_string: String    = input("Введите данные для программы: ")?;
    let expected_output: String = input("Что должна вывести программа: ")?;

    let spec = module6::ProblemSpec::new(expected_output);
    let input_data = module6::InputData::new(&path, &input_string, spec);

    if !input_data.is_valid() {
        return Err(CLIError::InvalidInputData);
    }

    Ok(input_data)
}
