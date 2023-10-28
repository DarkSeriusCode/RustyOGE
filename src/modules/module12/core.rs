use super::{FileInfo, FileSize};

use std::path::Path;

use unrar::{Archive, error::UnrarError};

pub fn get_files_in_dir(archive: Archive, folder: &Path) -> Result<Vec<FileInfo>, UnrarError> {
    let mut files = vec![];

    for file_entry in archive.open_for_listing()? {
        let file_entry = file_entry?;
        let file_name = &file_entry.filename;
        // Удаляем первую компоненту пути
        let file_path = file_name.strip_prefix(file_name.components().next().unwrap()).unwrap();
        if file_path.starts_with(folder) {
            files.push(FileInfo(file_entry.filename, FileSize::Bytes(file_entry.unpacked_size)));
        }
    }

    Ok(files)
}
