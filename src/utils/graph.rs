use std::collections::{LinkedList, HashMap, VecDeque};
use std::cmp::{PartialOrd, Ord, Ordering};

/// Представляет обычный взвешенный граф
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Graph(HashMap<String, Vec<(String, usize)>>);

impl Graph {
    /// Создаёт новый граф. В map ключ - имя вершины, значение - (имя соседней вершины, вес ребра
    /// до соседней вершины)
    pub fn new(map: HashMap<String, Vec<(String, usize)>>) -> Self {
        Self(map)
    }

    /// Ищет все возможные пути в графе от `src` до `dest`
    pub fn find_ways<'t>(&'t self, src: &'t str, dest: &'t str) -> LinkedList<Way> {
        let mut ways = LinkedList::from([Way::new(src)]);

        while ways.iter().any(|w| w.last().unwrap_or_default() != dest) {
            let mut new_ways = LinkedList::new();
            for way in ways {
                if way.last().unwrap() == dest {
                    new_ways.push_back(way);
                    continue;
                }
                for (neighbor, neighbor_edge) in &self.0[way.last().unwrap()] {
                    if !way.goes_through(neighbor) {
                        new_ways.push_back(Way::merge(&way, *neighbor_edge, &Way::new(neighbor)));
                    }
                }
            }
            ways = new_ways;
        }
        ways
    }
}

/// Представляет путь в графе
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Way<'a> {
    goes_through: VecDeque<&'a str>,
    len: usize,
}

impl<'a> Way<'a> {
    pub fn new(node: &'a str) -> Self {
        Self {
            goes_through: VecDeque::from([node]),
            len: 0,
        }
    }

    /// Возвращает длину пути
    pub fn length(&self) -> usize { self.len }

    /// Возвращает последнюю вершину пути
    pub fn last(&self) -> Option<&'a str> {
        self.goes_through.back().copied()
    }

    /// Соединяет lhs и rhs, с расстоянием между ними lhs_rhs_edge_w и возвращает полученный путь
    pub fn merge(lhs: &Way<'a>, lhs_rhs_edge_w: usize, rhs: &Way<'a>) -> Way<'a> {
        let mut new_way = lhs.clone();
        new_way.goes_through.extend(&rhs.goes_through);
        new_way.len += rhs.len + lhs_rhs_edge_w;
        new_way
    }

    /// Возвращает `true`, если путь прошодит через вершину `node`
    pub fn goes_through(&self, node: &'a str) -> bool {
        self.goes_through.contains(&node)
    }
}

impl<'a> PartialOrd for Way<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}

impl<'a> Ord for Way<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length().cmp(&other.length())
    }
}
