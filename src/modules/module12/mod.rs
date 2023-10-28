use std::ffi::OsStr;

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

    let (ext, size): (String, usize) = match input_data.spec {
        ProblemSpec::WithExtencion(ext)              => (ext, usize::MAX),
        ProblemSpec::WithExtencionAndSize(ext, size) => (ext, size.in_bytes()),
    };

    let filtered_files = all_files
        .iter()
        .filter(|FileInfo(path, fsize)| path.extension().unwrap_or(OsStr::new("")) == ext.as_str()
                                        && fsize.in_bytes() <= size);

    Ok(filtered_files.count().to_string())
}
