use std::path::PathBuf;

use rusty_oge::module6;
use rusty_oge::utils::Validated;

use crate::errors::CLIError;
use crate::utils::CLIResult;
use super::input_utils::*;

pub fn get_input() -> CLIResult<module6::InputData> {
    let path: PathBuf           = input("Введите путь до файла с программой: ")?;
    let input_string: String    = input("Введите данные для программы: ")?;
    let expected_output: String = input("Что должна вывести программа: ")?;

    let spec = module6::ProblemSpec::new(expected_output);
    let input_data = module6::InputData::new(&path, &input_string, spec);

    if let Err(validation_error_text) = input_data.valid() {
        return Err(CLIError::InvalidInputData(validation_error_text));
    }

    Ok(input_data)
}
