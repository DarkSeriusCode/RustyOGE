use pest::Parser;

use super::{LogicExpressionParser, Rule};
use crate::utils::{Validated, NumberToFind};

pub(super) type LogicFunc = Box<dyn Fn(u32) -> bool>;
pub(super) type Modificator = Box<dyn Fn(u32) -> u32>;

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Число, которое нужно найти (максимальное или минимальное)
    pub number_to_find: NumberToFind,
    /// Сколько цифр должно быть в искомом числе. `None` - если кол-во не важно
    /// Например, если нужно найти двузначное число, то `Some(2)`
    pub digits_in_number: Option<usize>,
    /// Данное в задаче выражение
    pub expression: String,
    /// Что должно вернуть выражение
    pub expression_result: bool,
}

impl InputData {
    pub fn new(num_to_find: NumberToFind, digits: Option<usize>,
               expr: &str, expr_res: bool) -> Self {
        Self {
            number_to_find: num_to_find,
            digits_in_number: digits,
            expression: expr.to_string(),
            expression_result: expr_res,
        }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        let parse_res = LogicExpressionParser::parse(Rule::main_expr, &self.expression);
        if let Err(e) = parse_res {
            return Err(e.to_string());
        }
        Ok(())
    }
}
