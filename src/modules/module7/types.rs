use std::collections::HashMap;

use crate::utils::Validated;

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Части IP-адреса. Ключ - буква, которой обозначен фрагмент ; Значение - часть IP-адреса
    pub ip_parts: HashMap<char, String>,
}

impl InputData {
    pub fn new(ip_parts: HashMap<char, String>) -> Self {
        Self { ip_parts, }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        for part in self.ip_parts.values() {
            if !part.chars().all(|ch| ch.is_numeric() || ch == '.') {
                return Err(format!("{} should contain only numbers and .", part));
            }
        }
        Ok(())
    }
}
