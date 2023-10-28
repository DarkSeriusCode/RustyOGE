extern crate rusty_oge;

use std::env;
use std::path::PathBuf;
use rusty_oge::module6::*;

#[macro_use]
mod test_macros;

const PROGRAMS_DIR_STR: &str = "tests/module6_files";

// ------------------------------------------------------------------------------------------------

fn path_to(fname: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(env::current_dir().unwrap());
    buf.push(PROGRAMS_DIR_STR);
    buf.push(fname);
    buf
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem10458,
    Input = (
        &path_to("10458.py"),
        "(1, 2); (11, 2); (1, 12); (11, 12); (–11, –12); (–11, 12); (–12, 11); (10, 10); (10, 5)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "5"
}

Test! {
    Name = problem10459,
    Input = (
        &path_to("10459.py"),
        "(1, 13); (14, 2); (1, 12); (11, 12); (–14, –14); (–11, 13); (–4, 11); (2, 9); (8, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "3"
}

Test! {
    Name = problem10460,
    Input = (
        &path_to("10460.py"),
        "(8, 8); (9, 6); (4, 7); (6, 6); (–9, –2); (–5, 9); (–10, 10); (6, 9); (10, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "5"
}

Test! {
    Name = problem10461,
    Input = (
        &path_to("10461.py"),
        "(8, 8); (9, 6); (4, 7); (6, 6); (–9, –2); (–5, 9); (–10, 10); (6, 9); (10, 6)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "4"
}

Test! {
    Name = problem10462,
    Input = (
        &path_to("10462.py"),
        "(9, 9); (9, 10); (8, 5); (11, 6); (–11, 10); (–5, 9); (–10, 10); (4, 5); (8, 6)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "5"
}

Test! {
    Name = problem10463,
    Input = (
        &path_to("10463.py"),
        "(9, 9); (9, 10); (8, 5); (11, 6); (–11, 10); (–5, 9); (–10, 10); (4, 5); (8, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "7"
}

Test! {
    Name = problem10464,
    Input = (
        &path_to("10464.py"),
        "(6, 4); (7, 8); (8, 5); (5, 6); (–11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "7"
}

Test! {
    Name = problem10465,
    Input = (
        &path_to("10465.py"),
        "(6, 4); (7, 8); (8, 5); (5, 6); (11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "8"
}

Test! {
    Name = problem10466,
    Input = (
        &path_to("10466.py"),
        "(6, 4); (7, 8); (12, 10); (5, 6); (11, 10); (–5, 7); (–2, 2); (4, 5); (8, 6)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "2"
}

Test! {
    Name = problem10467,
    Input = (
        &path_to("10467.py"),
        "(3, 4); (5, 4); (–2, 1); (5, 6); (7, 8); (–5, 5); (–2, 2); (4, 3); (3, –8)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "3"
}

Test! {
    Name = problem10468,
    Input = (
        &path_to("10468.py"),
        "(6, 8); (3, 5); (–7, 2); (7, 7); (9, 8); (–1, 3); (–4, 5); (6, 9); (2, –1)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "4"
}

Test! {
    Name = problem10469,
    Input = (
        &path_to("10469.py"),
        "(9, 10); (11, 5); (–2, 8); (9, 9); (2, 8); (–1, 3); (–4, 5); (10, 9); (4, –3)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "3"
}

Test! {
    Name = problem10470,
    Input = (
        &path_to("10470.py"),
        "(9, 10); (11, 5); (–2, 8); (9, 9); (2, 8); (–1, 3); (–4, 5); (10, 9); (4, –3)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "6"
}

Test! {
    Name = problem10471,
    Input = (
        &path_to("10471.py"),
        "(10, 10); (10, 6); (–4, 8); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (11, 11)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "8"
}

Test! {
    Name = problem10472,
    Input = (
        &path_to("10472.py"),
        "(10, 6); (7, 6); (–4, 3); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (6, 5)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "4"
}

Test! {
    Name = problem10888,
    Input = (
        &path_to("10888.py"),
        "(3, –3); (7, 6); (–4, 1); (2, 9); (12, 7); (–11, 4); (–8, 13); (10, 9); (6, 5)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "4"
}

Test! {
    Name = problem10890,
    Input = (
        &path_to("10890.py"),
        "(5, 3); (2, 4); (–1, 10); (5, 7); (5, 4); (–11, 4); (9, 13); (7, 9); (6, 8)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "2"
}

Test! {
    Name = problem10891,
    Input = (
        &path_to("10891.py"),
        "(7, 3); (2, 7); (6, 10); (5, 3); (5, 4); (–11, 4); (–8, 9); (7, 3); (9, 1)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "3"
}

Test! {
    Name = problem10892,
    Input = (
        &path_to("10892.py"),
        "(–2, 3); (2, 5); (0, 3); (5, –3); (5, 4); (11, 4); (8, –6); (7, 3); (9, 1)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "6"
}

Test! {
    Name = problem10893,
    Input = (
        &path_to("10893.py"),
        "(–2, 3); (2, 5); (0, 3); (5, –3); (5, 4); (11, 4); (8, –6); (1, 7); (9, 1)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "4"
}

Test! {
    Name = problem10950,
    Input = (
        &path_to("10950.py"),
        "(–1, 6); (2, 8); (0, 3); (9, –9); (4, 4); (2, 7); (8, –2); (7, 7); (4, 1)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "7"
}

Test! {
    Name = problem10951,
    Input = (
        &path_to("10951.py"),
        "(9, 5); (11, 2); (4, 5); (7, –2); (4, 4); (7, 7); (1, –1); (3, 9); (2, 2)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "6"
}

Test! {
    Name = problem10953,
    Input = (
        &path_to("10953.py"),
        "(2, 5); (5, 2); (4, 4); (2, –2); (3, 1); (8, 3); (9, –7); (7, 7); (4, 6)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "6"
}

Test! {
    Name = problem10954,
    Input = (
        &path_to("10954.py"),
        "(2, 5); (5, 2); (4, 4); (2, –2); (3, 1); (8, 3); (9, –7); (7, 7); (4, 6)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "3"
}

Test! {
    Name = problem10955,
    Input = (
        &path_to("10955.py"),
        "(9, 5); (11, 2); (4, 5); (7, –2); (4, 4); (7, 7); (1, –1); (3, 9); (2, 2)."
    ),
    Spec = ProblemSpec::new("NO".to_string()),
    Output = "3"
}

Test! {
    Name = problem12855,
    Input = (
        &path_to("12855.py"),
        "(3, 5); (4, 3); (4, -5); (0, 7); (0, -2); (-2, 1); (-2, 5); (-2, -4); (1, 2)."
    ),
    Spec = ProblemSpec::new("YES".to_string()),
    Output = "3"
}

Test! {
    Name = problem18174,
    Input = (
        &path_to("18174.py"),
        "(1, 1); (10, 7); (6, −12); (6, 6); (5, 2); (−10, −8); (−10, 11); (3, 1); (12, 8)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "6"
}

Test! {
    Name = problem18189,
    Input = (
        &path_to("18189.py"),
        "(1, 1); (10, 8); (9, −12); (6, 6); (5, 15); (−10, −8); (−10, 11); (3, 1); (1, 8)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "4"
}

Test! {
    Name = problem18215,
    Input = (
        &path_to("18215.py"),
        "(1, 1); (8, 4); (14, 10); (20, 1); (7, 3); (10, 5); (10, 2); (4, 1); (1, 0)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "4"
}

Test! {
    Name = problem18230,
    Input = (
        &path_to("18230.py"),
        "(1, 1); (8, 4); (14, 10); (20, 1); (7, 3); (10, 5); (10, 2); (4, 1); (1, 0)."
    ),
    Spec = ProblemSpec::new("НЕТ".to_string()),
    Output = "5"
}

Test! {
    Name = problem18245,
    Input = (
        &path_to("18245.py"),
        "(2, 2); (5, 9); (7, −12); (5, 5); (2, 12); (—10, —13); (—11, 11); (1, 4); (2, 6)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "7"
}

Test! {
    Name = problem18260,
    Input = (
        &path_to("18260.py"),
        "(2, 2); (5, 9); (7, −12); (5, 5); (2, 12); (—10, —13); (—11, 11); (1, 4); (2, 6)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "5"
}

Test! {
    Name = problem18275,
    Input = (
        &path_to("18275.py"),
        "(1, 2); (8, 4); (6, −12); (−5, −5); (3, 11); (—10, 12); (—10, −2); (4, 1); (2, 5)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "4"
}

Test! {
    Name = problem18290,
    Input = (
        &path_to("18290.py"),
        "(1, 2); (8, 4); (6, −12); (−5, −5); (3, 11); (−10, 12); (−10, −2); (4, 1); (2, 5)."
    ),
    Spec = ProblemSpec::new("ДА".to_string()),
    Output = "6"
}

