#[macro_export]
macro_rules! solve_problem {
    ($problem_num:literal, $matches:ident) => {
        paste::paste! {{
            use clap::{FromArgMatches};

            let input_data = $crate::cli_wrapers::[<module $problem_num>]
                ::[<Module $problem_num InputData>]::from_arg_matches($matches);
            if let Err(e) = input_data { e.exit() };
            let input_data = input_data.unwrap().0;

            rusty_oge::[<module $problem_num>]::solve(input_data).map_err(|e| e.into())
        }}
    }
}
