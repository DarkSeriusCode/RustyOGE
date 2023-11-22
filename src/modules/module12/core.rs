use super::{FileInfo, FileSize};

use std::path::Path;

use unrar::{Archive, error::UnrarError};

/// Возвращает список всех файлов в архиве, содержащих `folder` в качестве 1ой или 2ой компоненты
/// пути (Например: `folder`/other_folder_or_file или base_folder/`folder`/other_folder_or_file)
pub fn get_files_in_dir<P>(archive: Archive, folder: &P) -> Result<Vec<FileInfo>, UnrarError>
where
    P: AsRef<Path>
{
    let mut files = vec![];

    for file_entry in archive.open_for_listing()? {
        let file_entry = file_entry?;
        if file_entry.is_directory() { continue; }

        let file_name = &file_entry.filename;
        let has_folder = file_name.components().take(2).any(|c| c.as_os_str() == folder.as_ref());
        if has_folder {
            files.push(FileInfo::new(file_name, FileSize::Bytes(file_entry.unpacked_size)));
        }
    }

    Ok(files)
}
