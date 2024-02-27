use crate::utils::{SolveError, SolveResult, Validated};

use unrar::Archive;

mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    if let Err(validation_err) = input_data.valid() {
        return Err(SolveError(validation_err.into()));
    }

    let archive = Archive::new(&input_data.archive_path);
    let all_files = core::get_files_in_dir(archive, &input_data.search_dir)
        .map_err(SolveError::from)?;

    let exts = input_data.extensions;
    let size = input_data.minimum_file_size.unwrap_or_default();

    let filtered_files = all_files
        .iter()
        .filter(|finfo| exts.contains(&finfo.path.extension().unwrap_or_default().to_owned())
                                      && finfo.size >= size);

    Ok(filtered_files.count().to_string())
}
