use crate::utils::{graph::Graph, SolveError, SolveResult, Validated};

mod types;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    if let Err(validation_err) = input_data.valid() {
        return Err(SolveError(validation_err.into()));
    }

    let graph = Graph::new(input_data.map);
    let ways = graph.find_ways(&input_data.way.0, &input_data.way.1);
    let mut filtered_ways = ways;

    for node_name in input_data.include {
        filtered_ways = filtered_ways.iter()
            .filter(|w| w.goes_through(&node_name))
            .map(|w| w.to_owned())
            .collect();
    }
    for node_name in input_data.exclude {
        filtered_ways = filtered_ways.iter()
            .filter(|w| !w.goes_through(&node_name))
            .map(|w| w.to_owned())
            .collect();
    }

    Ok(filtered_ways.len().to_string())
}
