use std::path::{PathBuf, Path};
use std::ffi::OsString;

use crate::utils::Validated;

/// Детали решения задания. Указывает, по какому критерию искать файлы для подсчёта.
#[derive(Debug, Clone)]
pub enum ProblemSpec {
    /// В задаче нужно посчитать файлы с расширениями, указанными в `Vec<OsString>`
    WithExtencions(Vec<OsString>),
    /// В задаче нужно посчитать файлы с определённым расширением (`OsString`) и объёмом
    /// (`FileSize`).
    WithExtencionAndSize(OsString, FileSize),
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Путь до архива
    pub archive_path: PathBuf,
    /// Каталог в архиве, где нужно искть файлы
    pub search_dir: PathBuf,
    /// Детали решения задания.
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new<AP, SP>(archive_path: AP, search_dir: SP, spec: ProblemSpec) -> Self
    where
        AP: AsRef<Path>,
        SP: AsRef<Path>,
    {
        Self {
            archive_path: archive_path.as_ref().to_path_buf(),
            search_dir: search_dir.as_ref().to_path_buf(),
            spec,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if !self.archive_path.exists() {
            return Err(format!("Файла {} не существует!", &self.archive_path.to_str().unwrap()));
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

/// Базовая информация о файле, необходимая для поиска требуемых в задаче файлов.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: FileSize,
}

impl FileInfo {
    pub fn new<P: AsRef<Path>>(path: P, size: FileSize) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            size,
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Размер какого-либо файла.
#[derive(Debug, Copy, Clone)]
pub enum FileSize {
    /// Размер файла указан в байтах
    Bytes(usize),
    /// Размер файла указан в килобайтах
    Kb(usize),
    /// Размер файла указан в мегабайтах
    Mb(usize),
}

impl FileSize {
    /// Переводит размер файла в байты
    pub fn in_bytes(&self) -> usize {
        match self {
            Self::Bytes(n) => *n,
            Self::Kb(n)    => n * 1024,
            Self::Mb(n)    => n * 1048576,
        }
    }
}
