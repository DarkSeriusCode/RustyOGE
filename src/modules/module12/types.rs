use std::path::{PathBuf, Path};
use std::ffi::OsString;

use crate::utils::{
    Validated,
    data_size::DataSize
};

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Путь до архива
    pub archive_path: PathBuf,
    /// Каталог в архиве, где нужно искть файлы
    pub search_dir: PathBuf,
    /// Возможные расширения файла
    pub extensions: Vec<OsString>,
    /// Минимальный размер файла
    pub minimum_file_size: Option<DataSize>,
}

impl InputData {
    pub fn new<AP, SP>(archive_path: AP, search_dir: SP,
                       exts: Vec<OsString>, min_fsize: Option<DataSize>) -> Self
    where
        AP: AsRef<Path>,
        SP: AsRef<Path>,
    {
        Self {
            archive_path: archive_path.as_ref().to_path_buf(),
            search_dir: search_dir.as_ref().to_path_buf(),
            extensions: exts,
            minimum_file_size: min_fsize,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if !self.archive_path.exists() {
            return Err(format!("File {} doesn't exist", &self.archive_path.to_str().unwrap()));
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

/// Базовая информация о файле, необходимая для поиска требуемых в задаче файлов.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: DataSize,
}

impl FileInfo {
    pub fn new<P: AsRef<Path>>(path: P, size: DataSize) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            size,
        }
    }
}
