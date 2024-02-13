use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::utils::{self, SolveResult, SolveError};

mod types;

pub use types::*;

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    let parts = input_data.ip_parts;
    let all_key_combinations: Vec<String> = utils::combinations(parts.len(), parts.keys());

    for key_combination in all_key_combinations {
        let mut ip_addr = String::new();
        for key in key_combination.chars() {
            ip_addr.push_str(parts.get(&key).unwrap());
        }

        if Ipv4Addr::from_str(&ip_addr).is_ok() && utils::has_unique_chars(&key_combination) {
            return Ok(key_combination);
        }
    }

    Err(SolveError("Cannot make IP address from given parts!".into()))
}
