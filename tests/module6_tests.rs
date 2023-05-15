extern crate rusty_oge;

use std::env;
use std::path::PathBuf;
use rusty_oge::module6::*;
use rusty_oge::utils::Validated;


const PROGRAMS_DIR_STR: &str = "tests/module6_files";

// ------------------------------------------------------------------------------------------------

fn path_to(fname: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(env::current_dir().unwrap());
    buf.push(PROGRAMS_DIR_STR);
    buf.push(fname);
    buf
}

/// Пытается решить задание, если не получается - паникуем
fn try_solve(input_data: InputData, right_answer: &str) {
    match solve(input_data) {
        Ok(answer) => assert_eq!(answer, right_answer),
        Err(err) => panic!("{}", err),
    }
}

fn make_input_data(fname: &str, program_input: &str, excepted_output: &str) -> InputData {
    let input_data = InputData {
        file_path: path_to(fname),
        program_input: program_input.to_string(),
        spec: types::ProblemSpec { excepted_output: excepted_output.to_string(), },
    };
    assert!(input_data.is_valid());
    input_data
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem10458() {
    let input_data = make_input_data(
        "10458.py",
        "(1, 2); (11, 2); (1, 12); (11, 12); (–11, –12); (–11, 12); (–12, 11); (10, 10); (10, 5).",
        "YES"
    );
    try_solve(input_data, "5");
}

#[test]
fn problem10459() {
    let input_data = make_input_data(
        "10459.py",
        "(1, 13); (14, 2); (1, 12); (11, 12); (–14, –14); (–11, 13); (–4, 11); (2, 9); (8, 6).",
        "YES"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem10460() {
    let input_data = make_input_data(
        "10460.py",
        "(8, 8); (9, 6); (4, 7); (6, 6); (–9, –2); (–5, 9); (–10, 10); (6, 9); (10, 6).",
        "YES"
    );
    try_solve(input_data, "5");
}

#[test]
fn problem10461() {
    let input_data = make_input_data(
        "10461.py",
        "(8, 8); (9, 6); (4, 7); (6, 6); (–9, –2); (–5, 9); (–10, 10); (6, 9); (10, 6).",
        "NO"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem10462() {
    let input_data = make_input_data(
        "10462.py",
        "(9, 9); (9, 10); (8, 5); (11, 6); (–11, 10); (–5, 9); (–10, 10); (4, 5); (8, 6).",
        "NO"
    );
    try_solve(input_data, "5");
}

#[test]
fn problem10463() {
    let input_data = make_input_data(
        "10463.py",
        "(9, 9); (9, 10); (8, 5); (11, 6); (–11, 10); (–5, 9); (–10, 10); (4, 5); (8, 6).",
        "YES"
    );
    try_solve(input_data, "7");
}

#[test]
fn problem10464() {
    let input_data = make_input_data(
        "10464.py",
        "(6, 4); (7, 8); (8, 5); (5, 6); (–11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6).",
        "YES"
    );
    try_solve(input_data, "7");
}

#[test]
fn problem10465() {
    let input_data = make_input_data(
        "10465.py",
        "(6, 4); (7, 8); (8, 5); (5, 6); (11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6).",
        "YES"
    );
    try_solve(input_data, "8");
}

#[test]
fn problem10466() {
    let input_data = make_input_data(
        "10466.py",
        "(6, 4); (7, 8); (12, 10); (5, 6); (11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6).",
        "NO"
    );
    try_solve(input_data, "2");
}

#[test]
fn problem10467() {
    let input_data = make_input_data(
        "10467.py",
        "(3, 4); (5, 4); (–2, 1); (5, 6); (7, 8); (–5, 5); (–2, 2); (4, 3); (3, –8).",
        "NO"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem10468() {
    let input_data = make_input_data(
        "10468.py",
        "(6, 8); (3, 5); (–7, 2); (7, 7); (9, 8); (–1, 3); (–4, 5); (6, 9); (2, –1).",
        "YES"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem10469() {
    let input_data = make_input_data(
        "10469.py",
        "(9, 10); (11, 5); (–2, 8); (9, 9); (2, 8); (–1, 3); (–4, 5); (10, 9); (4, –3).",
        "YES"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem10470() {
    let input_data = make_input_data(
        "10470.py",
        "(9, 10); (11, 5); (–2, 8); (9, 9); (2, 8); (–1, 3); (–4, 5); (10, 9); (4, –3).",
        "NO"
    );
    try_solve(input_data, "6");
}

#[test]
fn problem10471() {
    let input_data = make_input_data(
        "10471.py",
        "(10, 10); (10, 6); (–4, 8); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (11, 11).",
        "NO"
    );
    try_solve(input_data, "8");
}

#[test]
fn problem10472() {
    let input_data = make_input_data(
        "10472.py",
        "(10, 6); (7, 6); (–4, 3); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (6, 5).",
        "NO"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem10888() {
    let input_data = make_input_data(
        "10888.py",
        "(3, –3); (7, 6); (–4, 1); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (6, 5).",
        "YES"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem10890() {
    let input_data = make_input_data(
        "10890.py",
        "(5, 3); (2, 4); (–1, 10); (5, 7); (5, 4); (–11, 4); (9, 13); (7, 9); (6, 8).",
        "YES"
    );
    try_solve(input_data, "2");
}

#[test]
fn problem10891() {
    let input_data = make_input_data(
        "10891.py",
        "(7, 3); (2, 7); (6, 10); (5, 3); (5, 4); (–11, 4); (–8, 9); (7, 3); (9, 1).",
        "YES"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem10892() {
    let input_data = make_input_data(
        "10892.py",
        "(–2, 3); (2, 5); (0, 3); (5, –3); (5, 4); (11, 4); (8, –6); (7, 3); (9, 1).",
        "YES"
    );
    try_solve(input_data, "6");
}

#[test]
fn problem10893() {
    let input_data = make_input_data(
        "10893.py",
        "(–2, 3); (2, 5); (0, 3); (5, –3); (5, 4); (11, 4); (8, –6); (1, 7); (9, 1).",
        "NO"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem10950() {
    let input_data = make_input_data(
        "10950.py",
        "(–1, 6); (2, 8); (0, 3); (9, –9); (4, 4); (2, 7); (8, –2); (7, 7); (4, 1).",
        "YES"
    );
    try_solve(input_data, "7");
}

#[test]
fn problem10951() {
    let input_data = make_input_data(
        "10951.py",
        "(9, 5); (11, 2); (4, 5); (7, –2); (4, 4); (7, 7); (1, –1); (3, 9); (2, 2).",
        "YES"
    );
    try_solve(input_data, "6");
}

#[test]
fn problem10953() {
    let input_data = make_input_data(
        "10953.py",
        "(2, 5); (5, 2); (4, 4); (2, –2); (3, 1); (8, 3); (9, –7); (7, 7); (4, 6).",
        "YES"
    );
    try_solve(input_data, "6");
}

#[test]
fn problem10954() {
    let input_data = make_input_data(
        "10954.py",
        "(2, 5); (5, 2); (4, 4); (2, –2); (3, 1); (8, 3); (9, –7); (7, 7); (4, 6).",
        "NO"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem10955() {
    let input_data = make_input_data(
        "10955.py",
        "(9, 5); (11, 2); (4, 5); (7, –2); (4, 4); (7, 7); (1, –1); (3, 9); (2, 2).",
        "NO"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem12855() {
    let input_data = make_input_data(
        "12855.py",
        "(3, 5); (4, 3); (4, -5); (0, 7); (0, -2); (-2, 1); (-2, 5); (-2, -4); (1, 2).",
        "YES"
    );
    try_solve(input_data, "3");
}

#[test]
fn problem18174() {
    let input_data = make_input_data(
        "18174.py",
        "(1, 1); (10, 7); (6, −12); (6, 6); (5, 2); (−10, −8); (−10, 11); (3, 1); (12, 8).",
        "ДА"
    );
    try_solve(input_data, "6");
}

#[test]
fn problem18189() {
    let input_data = make_input_data(
        "18189.py",
        "(1, 1); (10, 8); (9, −12); (6, 6); (5, 15); (−10, −8); (−10, 11); (3, 1); (1, 8).",
        "ДА"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem18215() {
    let input_data = make_input_data(
        "18215.py",
        "(1, 1); (8, 4); (14, 10); (20, 1); (7, 3); (10, 5); (10, 2); (4, 1); (1, 0).",
        "ДА"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem18230() {
    let input_data = make_input_data(
        "18230.py",
        "(1, 1); (8, 4); (14, 10); (20, 1); (7, 3); (10, 5); (10, 2); (4, 1); (1, 0).",
        "НЕТ"
    );
    try_solve(input_data, "5");
}

#[test]
fn problem18245() {
    let input_data = make_input_data(
        "18245.py",
        "(2, 2); (5, 9); (7, −12); (5, 5); (2, 12); (—10, —13); (—11, 11); (1, 4); (2, 6).",
        "ДА"
    );
    try_solve(input_data, "7");
}

#[test]
fn problem18260() {
    let input_data = make_input_data(
        "18260.py",
        "(2, 2); (5, 9); (7, −12); (5, 5); (2, 12); (—10, —13); (—11, 11); (1, 4); (2, 6).",
        "ДА"
    );
    try_solve(input_data, "5");
}

#[test]
fn problem18275() {
    let input_data = make_input_data(
        "18275.py",
        "(1, 2); (8, 4); (6, −12); (−5, −5); (3, 11); (—10, 12); (—10, −2); (4, 1); (2, 5).",
        "ДА"
    );
    try_solve(input_data, "4");
}

#[test]
fn problem18290() {
    let input_data = make_input_data(
        "18290.py",
        "(1, 2); (8, 4); (6, −12); (−5, −5); (3, 11); (−10, 12); (−10, −2); (4, 1); (2, 5).",
        "ДА"
    );
    try_solve(input_data, "6");
}

