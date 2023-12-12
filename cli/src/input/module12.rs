use std::path::PathBuf;
use std::ffi::OsString;

use rusty_oge::module12;
use rusty_oge::utils::{Validated, data_size::DataSize};

use crate::errors::{CLIResult, CLIError};
use crate::utils::Pair;
use super::input_utils::*;


pub fn get_input() -> CLIResult<module12::InputData> {
    let archive_path: PathBuf = input("Введите путь до архива:")?;
    let search_folder: PathBuf = input("Введите каталог в котором нужно подсчитать файлы:")?;
    let spec = get_spec()?;

    let input_data = module12::InputData::new(archive_path, search_folder, spec);

    if let Err(validation_error_text) = input_data.valid() {
        return Err(CLIError::InvalidInputData(validation_error_text));
    }
    Ok(input_data)
}

fn get_spec() -> CLIResult<module12::ProblemSpec> {
    let spec_has_fsize = choose("Какие файлы нужно найти?", &[
        ("Только с определёнными расширениями",     &false),
        ("С определёнными расширениями и размером", &true),
    ])?;
    let exts: Vec<OsString> = input_until_end("Введите расширения")?;
    let mut spec = module12::ProblemSpec::WithExtencions(exts.clone());

    if *spec_has_fsize {
        let ext = exts.first().cloned().unwrap_or(OsString::new());
        spec = module12::ProblemSpec::WithExtencionAndSize(ext, get_file_size()?);
    }

    Ok(spec)
}

fn get_file_size() -> CLIResult<DataSize> {
    let raw_fsize: Pair<usize, String> = input("Введите размер и еденицу измерения (B, Kb, Mb)\
                                                через пробел (пример: 2 Mb):")?;
    match raw_fsize.second().to_lowercase().as_str() {
        "b"  => Ok(DataSize::bytes(*raw_fsize.first())),
        "kb" => Ok(DataSize::kb(*raw_fsize.first())),
        "mb" => Ok(DataSize::mb(*raw_fsize.first())),
        _    => Err(CLIError::IncorrectInput("Неверная единица измерения! Возможные варианты: B, \
                                             Kb, Mb".into())),
    }
}
