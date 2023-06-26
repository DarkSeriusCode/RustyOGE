use std::{fmt, error, cmp};

/// Результат конвертирования числа
pub type ConvertionResult = Result<Number, ConvertionError>;

/// Перечисление ошибок, возникаемых при конвертации
#[derive(Debug)]
pub enum ConvertionError {
    /// Основание больше 36
    VeryBigBase,
    /// Число содержит недопустимые в данной системе счисления символы, или число не корректно
    InvalidInteger,
}

impl ConvertionError {
    /// Возвращает сообщение ошибки
    pub fn message(&self) -> &str {
        match self {
            Self::VeryBigBase    => "Основание больше 36",
            Self::InvalidInteger => "Некоректное число",
        }
    }
}

impl error::Error for ConvertionError {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Display for ConvertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
/// Представляет число с неким основанием
pub struct Number {
    number: String,
    base: u32,
}

impl Number {
    /// Создаёт число с указанным основанием
    pub fn new(number: &str, base: u32) -> Self {
         Self {
            number: number.to_string(),
            base,
        }
    }

    /// Переводит число в систему счисления с основанием `base`
    pub fn convert(&self, base: u32) -> Result<Self, ConvertionError> {
        let is_negative = self.number.starts_with('-').into();
        if base > 36 { return Err(ConvertionError::VeryBigBase); }

        // Переводим сначала в десятичню. Если значение есть в кэше - берём его
        let in_decimal = Self::to_decimal(&self.number, self.base)?;

        // Переводим число в число с необходимым основанием
        let mut in_needed_base = Self::from_decimal(in_decimal, base)?;
        if is_negative {
            in_needed_base.insert(0, '-');
        }

        Ok(Self {
            number: in_needed_base,
            base,
        })
    }

    pub fn number(&self) -> String {
        self.number.clone()
    }

    pub fn base(&self) -> u32 {
        self.base
    }

    /// Переводит не отрицательное число с основанием `base` в десятичную систему счисления
    ///
    /// # Паникует
    /// Если `base` больше 36!
    fn to_decimal(number_str: &str, base: u32) -> Result<u32, ConvertionError> {
        u32::from_str_radix(number_str, base).map_err(|_| ConvertionError::InvalidInteger)
    }

    /// Переводит не отрицательное число в десятичной системе счисления в число с основанием `base`
    ///
    /// # Паникует
    /// Если `base` больше 36
    fn from_decimal(mut number: u32, base: u32) -> Result<String, ConvertionError> {
        let mut buff = vec![];

        while number != 0 {
            let ch = char::from_digit(number % base, base)
                .ok_or_else(|| ConvertionError::InvalidInteger)?;
            buff.push(ch);
            number /= base;
        }
        Ok(buff.iter().rev().collect())
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.convert(10).unwrap().number == other.convert(10).unwrap().number
    }
}

impl cmp::Eq for Number {}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // Избавиться от этих постоянных unwrap'ов
        let self_num: i32 = self.convert(10).unwrap().number().parse().unwrap();
        let other_num: i32 = other.convert(10).unwrap().number().parse().unwrap();
        self_num.partial_cmp(&other_num)
    }
}

impl cmp::Ord for Number {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

