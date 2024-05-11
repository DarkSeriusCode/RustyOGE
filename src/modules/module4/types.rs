use std::collections::HashMap;

use crate::utils::Validated;

/// Какой путь необходимо найти
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathToFind {
    /// Нужно найти самый короткий путь
    Shortest,
    /// Нужно найти самый длинный путь
    Longest,
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Данный в задаче граф, где ключ - имя вершины, значение - (имя соседней вершиныб расстояние
    /// до неё)
    pub map: HashMap<String, Vec<(String, usize)>>,
    /// Между какими вершинами нужно найти кратчайший путь
    pub way: (String, String),
    /// Через какие вершины обязательно нужно пройти
    pub include: Vec<String>,
    pub path_to_find: PathToFind,
}

impl InputData {
    pub fn new(map: HashMap<String, Vec<(String, usize)>>, way: (&str, &str),
               include: Vec<String>, path_to_find: PathToFind) -> Self
    {

        Self {
            map,
            way: (way.0.to_string(), way.1.to_string()),
            include: Vec::from_iter(include.iter().map(|s| s.to_string())),
            path_to_find,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if !self.map.contains_key(&self.way.0) {
            return Err(format!("Given map has no node with name \"{}\"", self.way.0));
        }
        if !self.map.contains_key(&self.way.1) {
            return Err(format!("Given map has no node with name \"{}\"", self.way.1));
        }

        for node_name in &self.include {
            if !self.map.contains_key(node_name) {
                return Err(format!("\"include\" contains node \"{}\" but map doesn't", node_name));
            }
        }

        Ok(())
    }
}
