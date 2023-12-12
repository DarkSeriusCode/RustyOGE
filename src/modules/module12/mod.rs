use std::ffi::OsString;
use crate::{SolveError, SolveResult};
use crate::utils::data_size::DataSize;

use unrar::Archive;

mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let archive = Archive::new(&input_data.archive_path);
    let all_files = core::get_files_in_dir(archive, &input_data.search_dir)
        .map_err(|e| SolveError::from(e))?;

    let (exts, size): (Vec<OsString>, DataSize) = match input_data.spec {
        ProblemSpec::WithExtencions(exts)            => (exts, DataSize::default()),
        ProblemSpec::WithExtencionAndSize(ext, size) => (vec![ext], size),
    };

    let filtered_files = all_files
        .iter()
        .filter(|finfo| exts.contains(&finfo.path.extension().unwrap_or_default().to_owned())
                                      && finfo.size >= size);

    Ok(filtered_files.count().to_string())
}
