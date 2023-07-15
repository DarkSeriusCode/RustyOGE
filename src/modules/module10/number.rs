use std::{fmt, error, cmp};

/// Перечисление ошибок, возникаемых при конвертации
#[derive(Debug)]
pub enum ConvertionError {
    /// Основание больше 36
    VeryBigBase,
    /// Число содержит недопустимые в данной системе счисления символы, или число не корректно
    InvalidInteger,
}

impl ConvertionError {
    /// Возвращает сообщение об ошибке
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
/// Представляет число в системе счисления с неким основанием
pub struct Number {
    number: String,
    base: u32,
}

impl Number {
    /// Создаёт число в заданной системе счисления. Также число можно создать,
    /// используя [макрос `num`](crate::num)
    pub fn new(number: &str, base: u32) -> Result<Self, ConvertionError> {
        let num = Self {
            number: number.to_string(),
            base,
        };

        num.convert(base)
    }

    /// Переводит число в систему счисления с основанием `base`
    pub fn convert(&self, base: u32) -> Result<Self, ConvertionError> {
        if base > 36 { return Err(ConvertionError::VeryBigBase); }

        let is_negative = self.number.starts_with('-');
        let number = if is_negative { &self.number[1..] } else { &self.number };

        // Переводим сначала в десятичню.
        let in_decimal = Self::to_decimal(number, self.base)?;

        // Переводим число в число с необходимым основанием
        let mut in_needed_base = Self::from_decimal(in_decimal, base)?;
        if is_negative { in_needed_base.insert(0, '-'); }

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

    /// Переводит не отрицательное число из десятичной системе счисления в систему счисления с
    /// основанием `base`
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
        let self_num = self.convert(10);
        let other_num = other.convert(10);

        if self_num.is_err() || other_num.is_err() { return false; }
        self_num.unwrap().number() == other_num.unwrap().number()
    }
}

impl cmp::Eq for Number {}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let self_num: i32 = self.convert(10).ok()?.number().parse().ok()?;
        let other_num: i32 = other.convert(10).ok()?.number().parse().ok()?;
        self_num.partial_cmp(&other_num)
    }
}

impl cmp::Ord for Number {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Создаёт [`Number`](crate::module10::Number), если не удаётся - паникует.
///
/// ## Пример
/// ```rust
/// use rusty_oge::num;
///
/// // Не паникует, число корректно
/// let valid_num = num!("10", 2);
///
/// // Паника! В двоичной системе счисления нет цифры 8
/// // let invalid_num = num!("18", 2);
/// ```
#[macro_export]
macro_rules! num {
    ($num:literal, $base:literal) => {
        $crate::module10::Number::new($num, $base)
            .expect(&format!("Invalid Number! (\"{}\", {})", $num, $base))
    };
}

