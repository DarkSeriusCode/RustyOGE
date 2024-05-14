use std::collections::HashMap;

use crate::utils::Validated;

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Данный в задаче граф, где ключ - имя вершины, значение - (имя соседней вершиныб расстояние
    /// до неё)
    pub map: HashMap<String, Vec<(String, usize)>>,
    /// Между какими вершинами нужно найти количество путей
    pub way: (String, String),
    /// Вершины, через которые должны проходить искомые пути
    pub include: Vec<String>,
    /// Вершины, через которые не должны проходить искомые пути
    pub exclude: Vec<String>,
}

impl InputData {
    pub fn new(map: HashMap<String, Vec<String>>, way: (&str, &str), include: Vec<String>,
               exclude: Vec<String>) -> Self
    {
        let mut weighted_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();
        for (key, value) in map {
            let value = Vec::from_iter(value.iter().map(|s| (s.to_string(), 0)));
            weighted_map.insert(key.to_string(), value);
        }

        Self {
            map: weighted_map,
            way: (way.0.to_string(), way.1.to_string()),
            include,
            exclude,
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
        for node_name in &self.exclude {
            if !self.map.contains_key(node_name) {
                return Err(format!("\"exclude\" contains node \"{}\" but map doesn't", node_name));
            }
        }
        Ok(())
    }
}
