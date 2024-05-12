use std::{
    cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering},
    default::Default,
    fmt::Display,
    str::FromStr,
    error::Error,
};

use regex::Regex;

/// Единицы измерения информации
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DataSizeUnit {
    Bytes,
    Kb,
    Mb,
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseDataSizeError {
    InvalidFormat,
    UnknownDataUnit(String),
}

impl Display for ParseDataSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseDataSizeError::*;
        let txt = match self {
            InvalidFormat => "Invalid format! Format as <num >= 0><unit>".into(),
            UnknownDataUnit(unit) => format!("Unknown data unit {unit}"),
        };
        write!(f, "{}", txt)
    }
}

impl Error for ParseDataSizeError {}

// ------------------------------------------------------------------------------------------------

/// Размер какой-либо информации
#[derive(Debug, Clone, Copy)]
pub struct DataSize {
    unit: DataSizeUnit,
    size: usize,
}

impl DataSize {
    pub fn new(size: usize, unit: DataSizeUnit) -> Self { Self { size, unit } }
    /// Алиас для `DataSize::new(size, DataSizeUnit::Bytes)`
    pub fn bytes(size: usize) -> Self { Self { size, unit: DataSizeUnit::Bytes } }
    /// Алиас для `DataSize::new(size, DataSizeUnit::Kb)`
    pub fn kb(size: usize) -> Self { Self { size, unit: DataSizeUnit::Kb } }
    /// Алиас для `DataSize::new(size, DataSizeUnit::Mb)`
    pub fn mb(size: usize) -> Self { Self { size, unit: DataSizeUnit::Mb } }

    pub fn size(&self) -> usize { self.size }
    pub fn unit(&self) -> DataSizeUnit { self.unit }

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
            Bytes => "b",
            Kb    => "kb",
            Mb    => "mb",
        };

        write!(f, "{}{}", self.size, postfix)
    }
}

impl Default for DataSize {
    fn default() -> Self {
        Self::bytes(0)
    }
}

impl FromStr for DataSize {
    type Err = ParseDataSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?<num>\d+)(?<unit>\w{1, 2})").expect("Cannot create Regex!");

        let Some(capture) = re.captures(s) else { return Err(Self::Err::InvalidFormat); };
        let num: usize = capture["num"].parse().unwrap();
        let unit = match &capture["unit"] {
            "B"  => DataSizeUnit::Bytes,
            "Kb" => DataSizeUnit::Kb,
            "Gb" => DataSizeUnit::Mb,
            unit => return Err(Self::Err::UnknownDataUnit(unit.to_string())),
        };

        Ok(DataSize::new(num, unit))
    }
}
