use super::CommandTable;
use crate::utils;

/// Выполняет программу с начальным числом `beg_value` и значением переменной `var_value`
pub fn execute_program(cmds: &CommandTable, program: &str, beg_value: i32, var_value: i32) -> f32 {
    let mut acc = beg_value as f32;

    for cmd_name in program.chars() {
        let cmd = cmds.get(&cmd_name).unwrap();
        acc = cmd.execute(acc, var_value as f32);
    }
    acc
}

/// Находит значение переменной, при котором программа `program` из числа `beg_value` получает
/// `res_value`
pub fn find_variable_value(cmds: &CommandTable, program: &str,
                           beg_value: i32, res_value: i32) -> Option<i32> {
    for b in 0..i32::MAX {
        if res_value as f32 == execute_program(cmds, program, beg_value, b) {
            return Some(b)
        }
    }
    None
}

/// Состовляет алгоритм длиной в `len` команд (`cmds`), который переодит число `beg_value` в
/// `res_value` и возвращает None, если это невозможно
pub fn make_algorithm(cmds: &CommandTable, len: usize,
                      beg_value: i32, res_value: i32) -> Option<String> {
    for program in utils::combinations::<String, _, _>(len, cmds.keys()) {
        // 1 потому что в задачах с составлением алгоритма переменной b нет
        if res_value as f32 == execute_program(cmds, &program, beg_value, 1) {
            return Some(program);
        }
    }
    None
}
