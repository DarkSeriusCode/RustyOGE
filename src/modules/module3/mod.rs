use std::ops::{AddAssign, SubAssign};

use pest::Parser;

use crate::utils::{SolveResult, SolveError, NumberToFind, Validated};

mod types;
mod core;

pub use types::*;
use core::{Rule, LogicExpressionParser};

/// Решает задачу и возвращает результат решения.
pub fn solve(input_data: InputData) -> SolveResult {
    if let Err(validation_err) = input_data.valid() {
        return Err(SolveError(validation_err.into()));
    }

    let parse_result = LogicExpressionParser::parse(Rule::main_expr, &input_data.expression)
        .unwrap().next().unwrap();

    let logic_func = core::make_logic_func(parse_result);

    let mut variable: u32 = match (input_data.number_to_find, input_data.digits_in_number) {
        (NumberToFind::Max, Some(digits_count)) => 10u32.pow(digits_count as u32) - 1,
        (NumberToFind::Max, None)               => 10_000u32,
        (NumberToFind::Min, Some(digits_count)) => 10u32.pow(digits_count as u32 - 1) - 1,
        (NumberToFind::Min, None)               => 1u32,
    };
    let op = match input_data.number_to_find {
        NumberToFind::Max => SubAssign::sub_assign,
        NumberToFind::Min => AddAssign::add_assign,
    };

    while logic_func(variable) != input_data.expression_result {
        op(&mut variable, 1);
    }

    Ok(variable.to_string())
}
