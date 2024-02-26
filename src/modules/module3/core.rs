use pest::iterators::Pair;
use pest_derive::Parser;

use super::{LogicFunc, Modificator};

#[derive(Debug, Clone, Parser)]
#[grammar = "modules/module3/grammar.pest"]
pub(super) struct LogicExpressionParser;

// ------------------------------------------------------------------------------------------------

pub fn make_logic_func(pair: Pair<'_, Rule>) -> LogicFunc {
    match pair.as_rule() {
        Rule::and_expr => {
            let mut and_expr_inner = pair.into_inner();
            let lhs = and_expr_inner.next().unwrap();  // Left expr
            let rhs = and_expr_inner.next().unwrap();  // Right expr
            return make_and(make_logic_func(lhs), make_logic_func(rhs));
        },
        Rule::or_expr => {
            let mut or_expr_inner = pair.into_inner();
            let lhs = or_expr_inner.next().unwrap();  // Left expr
            let rhs = or_expr_inner.next().unwrap();  // Right expr
            return make_or(make_logic_func(lhs), make_logic_func(rhs));
        },
        Rule::not_expr => {
            let mut not_expr_inner = pair.into_inner();
            return make_not(make_logic_func(not_expr_inner.next().unwrap()));
        },
        Rule::cmp_expr => {
            let mut cmp_expr_inner = pair.into_inner();
            let var_moficator = make_moduficator_from_pair(cmp_expr_inner.next().unwrap());
            let cmp_op = cmp_expr_inner.next().unwrap().as_str();
            let num: u32 = cmp_expr_inner.next().unwrap().as_str().parse().unwrap();
            return make_cmp_func(var_moficator, cmp_op.to_string(), num);
        },
        Rule::even_odd_expr => {
            let mut even_odd_expr_inner = pair.into_inner();
            let var_moficator = make_moduficator_from_pair(even_odd_expr_inner.next().unwrap());
            let is_even = if even_odd_expr_inner.as_str().starts_with("четн") { true }
                          else if even_odd_expr_inner.as_str().starts_with("нечетн") { false }
                          else { unreachable!() };
            return make_even_odd_func(var_moficator, is_even);
        },
        Rule::div_expr => {
            let mut div_expr_inner = pair.into_inner();
            let var_moficator = make_moduficator_from_pair(div_expr_inner.next().unwrap());
            let div_num: u32 = div_expr_inner.next().unwrap().as_str().parse().unwrap();
            return make_div_func(var_moficator, div_num);
        }
        _ => unreachable!(),
    }
}

// // ------------------------------------------------------------------------------------------------

fn make_moduficator_from_pair(pair: Pair<'_, Rule>) -> Modificator {
    let Some(value) = pair.into_inner().next()
        else { return Box::new(|var: u32| var) };

    match value.as_rule() {
        Rule::digit => {
            let digit_pos = value.into_inner().next().unwrap();
            let index = match digit_pos.as_str() {
                "первая"    => 0,
                "вторая"    => 1,
                "последняя" => digit_pos.as_str().len(),
                &_          => unreachable!(),
            };
            return Box::new(move |var: u32| var.to_string()
                .chars().nth(index).unwrap_or('0')
                .to_string().parse().unwrap());
        },
        _ => unreachable!(),
    }
}

fn make_div_func(modificator: Modificator, div_num: u32) -> LogicFunc {
    Box::new(move |var: u32| modificator(var) % div_num == 0)
}

fn make_even_odd_func(modificator: Modificator, is_even: bool) -> LogicFunc {
    // Box::new(move |var: u32| modificator(var) % 2 == 0 && is_even)
    Box::new(move |var: u32| if is_even { modificator(var) % 2 == 0 }
                             else { modificator(var) % 2 != 0 })
}

fn make_cmp_func(modificator: Modificator, cmp_op: String, num: u32) -> LogicFunc {
    Box::new(move |var: u32| match cmp_op.as_str() {
        "<"  => modificator(var) < num,
        "<=" => modificator(var) <= num,
        ">"  => modificator(var) > num,
        ">=" => modificator(var) >= num,
        &_   => unreachable!()
    })
}

fn make_not(f: LogicFunc) -> LogicFunc {
    Box::new(move |var: u32| !f(var))
}

fn make_and(f1: LogicFunc, f2: LogicFunc) -> LogicFunc {
    Box::new(move |var: u32| f1(var) && f2(var))
}

fn make_or(f1: LogicFunc, f2: LogicFunc) -> LogicFunc {
    Box::new(move |var: u32| f1(var) || f2(var))
}
