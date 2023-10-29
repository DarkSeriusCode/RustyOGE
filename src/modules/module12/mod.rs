use std::ffi::OsString;
use crate::{SolveError, SolveResult};

use unrar::Archive;

pub mod types;
mod core;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let archive = Archive::new(&input_data.archive_path);
    let all_files = core::get_files_in_dir(archive, &input_data.search_dir)
        .map_err(|e| SolveError::from(e))?;

    let (exts, size): (Vec<OsString>, usize) = match input_data.spec {
        ProblemSpec::WithExtencions(exts)            => (exts, usize::MIN),
        ProblemSpec::WithExtencionAndSize(ext, size) => (vec![ext], size.in_bytes()),
    };

    let filtered_files = all_files
        .iter()
        .filter(|FileInfo(path, fsize)| exts.contains(&path.extension().unwrap_or_default().to_owned())
                                        && fsize.in_bytes() >= size);

    Ok(filtered_files.count().to_string())
}
