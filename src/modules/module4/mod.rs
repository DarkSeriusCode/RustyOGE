use crate::utils::{graph::Graph, SolveResult, SolveError};

mod types;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let graph = Graph::new(input_data.map);
    let ways = graph.find_ways(&input_data.way.0, &input_data.way.1);
    let mut filtered_ways = ways;

    for node_name in input_data.include {
        filtered_ways = filtered_ways.iter()
            .filter(|w| w.goes_through(&node_name))
            .map(|w| w.to_owned())
            .collect();
    }

    if filtered_ways.is_empty() {
        return Err(SolveError(format!("Cannot find the sortest way from {} to {}",
                                      input_data.way.0, input_data.way.1).into()))
    }

    let needed_path = match input_data.path_to_find {
        PathToFind::Longest  => filtered_ways.iter().max(),
        PathToFind::Shortest => filtered_ways.iter().min(),
    }.unwrap().length();

    Ok(needed_path.to_string())
}
