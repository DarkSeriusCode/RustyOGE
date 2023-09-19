use std::fmt::Display;
use std::error::Error;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

/// Перечисление ошибок, возникаемых при конвертации
#[derive(Debug)]
pub enum ConvertionError {
    /// Основание больше 36
    VeryBigBase,
    /// Число содержит недопустимые в данной системе счисления символы, или число не корректно
    InvalidInteger,
}

impl Display for ConvertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            Self::VeryBigBase    => "Основание больше 36",
            Self::InvalidInteger => "Некоректное число",
        };
        write!(f, "{err_msg}")
    }
}

impl Error for ConvertionError {}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
/// Представляет число в системе счисления с неким основанием
///
/// # Пример
/// ```rust
/// use rusty_oge::module10::Number;
///
/// let my_num_in_hex = Number::new("ABC", 16).unwrap();
/// let my_num_in_dec = my_num_in_hex.convert(10).expect("Cannot convert!");
///
/// // ABC в десятичной системе счисления это 2748
/// assert_eq!(my_num_in_dec.number(), "2748");
///
/// // Числа равны, т.к сравниваются их записи в десятичной системе счисления
/// assert_eq!(my_num_in_hex, my_num_in_dec);
/// ```
pub struct Number {
    number: String,
    base: u32,
}

impl Number {
    /// Создаёт число в заданной системе счисления.
    ///
    /// Если число не корректно (не соответствует системе счисления), возвращает `Err`.
    /// Макрос [`num`](crate::num) также создаёт `Number`, но паникует при не корректном числе.
    ///
    /// # Пример
    /// ```rust
    /// use rusty_oge::{num, module10::Number};
    ///
    /// let valid_num = Number::new("3C", 16);
    /// assert!(valid_num.is_ok());
    ///
    /// let invalid_num = Number::new("19", 8);
    /// assert!(invalid_num.is_err()); // Т.к в восьмеричной системе счисления нет цифры 9
    /// ```
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

        Ok(Self { number: in_needed_base, base })
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

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let self_num = self.convert(10);
        let other_num = other.convert(10);

        if self_num.is_err() || other_num.is_err() { return false; }
        self_num.unwrap().number() == other_num.unwrap().number()
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_num: i32 = self.convert(10).ok()?.number().parse().ok()?;
        let other_num: i32 = other.convert(10).ok()?.number().parse().ok()?;
        self_num.partial_cmp(&other_num)
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

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
