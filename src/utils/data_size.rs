use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::default::Default;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
/// Единицы измерения информации
pub enum DataSizeUnit {
    Bytes,
    Kb,
    Mb,
}

#[derive(Debug, Clone, Copy)]
/// Размер какой-либо информации
pub struct DataSize {
    unit: DataSizeUnit,
    size: usize,
}

impl DataSize {
    pub fn new(size: usize, unit: DataSizeUnit) -> Self {
        Self { size, unit }
    }

    /// Алиас для `DataSize::new(size, DataSizeUnit::Bytes)`
    pub fn bytes(size: usize) -> Self {
        Self { size, unit: DataSizeUnit::Bytes }
    }

    /// Алиас для `DataSize::new(size, DataSizeUnit::Kb)`
    pub fn kb(size: usize) -> Self {
        Self { size, unit: DataSizeUnit::Kb }
    }

    /// Алиас для `DataSize::new(size, DataSizeUnit::Mb)`
    pub fn mb(size: usize) -> Self {
        Self { size, unit: DataSizeUnit::Mb }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn unit(&self) -> DataSizeUnit {
        self.unit
    }

    /// Переводит размер в другие единицы
    pub fn in_unit(&self, unit: DataSizeUnit) -> Self {
        Self::new(self.in_bytes() / 1024_usize.pow(unit as u32), unit)
    }

    pub fn in_bytes(&self) -> usize {
        self.size * 1024_usize.pow(self.unit as u32)
    }
}

impl PartialEq for DataSize {
    fn eq(&self, other: &Self) -> bool {
        self.in_bytes() == other.in_bytes()
    }
}

impl Eq for DataSize {}

impl PartialOrd for DataSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.in_bytes().partial_cmp(&other.in_bytes())
    }
}

impl Ord for DataSize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataSizeUnit::*;
        let postfix = match self.unit {
            Bytes => "B",
            Kb    => "Kb",
            Mb    => "Mb",
        };

        write!(f, "{}{}", self.size, postfix)
    }
}

impl Default for DataSize {
    fn default() -> Self {
        Self::bytes(0)
    }
}
