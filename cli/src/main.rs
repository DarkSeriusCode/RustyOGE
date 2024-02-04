use std::error::Error;

use clap::{
    crate_authors, crate_version, value_parser,
    Arg, ArgAction, ArgMatches, Command,
    error::ErrorKind,
};
use color_print::{cprintln, cformat};

mod utils;
mod cli_wrapers;
#[macro_use]
mod macros;

use utils::{CommandArgMixin, exit_with_any_error};

const AVAILABLE_PROBLEMS: [u32;5] = [1, 2, 6, 10, 12];
const PRINT_ERROR: &str = "FATAL ERROR while printing the text";

// ------------------------------------------------------------------------------------------------

fn main() {
    let mut cli = Command::new("rusty_oge")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Program that helps you to solve ОГЭ problems.")
        .dont_delimit_trailing_values(true)
        .disable_help_subcommand(true)
        .disable_help_flag(true)
        .subcommands([
            Command::new("list")
                .about("Print a list of problems that can be solved by the program."),
            Command::new("solve")
                .about("Solve some problem.")
                .args([
                    Arg::new("problem_num")
                        .long("problem_num")
                        .short('p')
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("Number of the problem to solve."),
                ]),
        ])
        .args([
            Arg::new("help_arg")
                .long("help")
                .short('h')
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Print help message about something"),
        ]);

    cli = mix_args_to_solve_if_needed(cli);

    let raw_matches = cli.clone().ignore_errors(true).get_matches();
    if *raw_matches.get_one::<bool>("help_arg").unwrap_or(&false) {
        match raw_matches.subcommand_name() {
            Some(name) => cli.find_subcommand_mut(name).unwrap().print_help().expect(PRINT_ERROR),
            None => cli.print_help().expect(PRINT_ERROR),
        };
        return;
    }

    let matches = cli.clone().get_matches();

    match matches.subcommand() {
        Some(("list",    _)) => show_list_of_available_problems(),
        Some(("solve",   solve_m)) => solve(solve_m),
        _ => cli.print_help().expect(PRINT_ERROR),
    }
}

// ------------------------------------------------------------------------------------------------

fn solve(matches: &ArgMatches) {
    let problem_num = *matches.get_one::<u32>("problem_num").unwrap();

    let solve_res: Result<String, Box<dyn Error>> = match problem_num {
        1 => solve_problem!(1, matches),
        2 => solve_problem!(2, matches),
        6 => solve_problem!(6, matches),
        10 => solve_problem!(10, matches),
        12 => solve_problem!(12, matches),
        _ => unreachable!(),
    };

    let Ok(answer) = solve_res else { exit_with_any_error(solve_res.unwrap_err()) };
    cprintln!("Answer: <g><s>{}", answer);
}

// ------------------------------------------------------------------------------------------------

/// Получаем номер задачи, которую нужно решить и в зависимости от номера подмешиваем
/// соответствующие примеси, но только если не активирован интерактивный режим!
fn mix_args_to_solve_if_needed(cmd: Command) -> Command {
    let matches = cmd.clone().ignore_errors(true).get_matches();
    let cmd = cmd.ignore_errors(false);
    let Some(solve_m) = matches.subcommand_matches("solve") else { return cmd };

    // Завершится с ошибкой, если не указан problem_num, а problem_num - обязательный
    let Some(problem_num) = solve_m.get_one::<u32>("problem_num")
        else { cmd.try_get_matches().unwrap_err().exit() };
    if !AVAILABLE_PROBLEMS.contains(problem_num) {
        clap::Error::raw(ErrorKind::InvalidValue,
                         format!("There's no problem with №{}!", *problem_num)).exit()
    }

    // Перезаписываем подкоманду `solve`, на новую с примесями
    cmd.mut_subcommand("solve", |subcmd| {
        use cli_wrapers::*;

        match problem_num {
            1  => module1::Module1InputData::mix_to_command(subcmd),
            2  => module2::Module2InputData::mix_to_command(subcmd),
            6  => module6::Module6InputData::mix_to_command(subcmd),
            10 => module10::Module10InputData::mix_to_command(subcmd),
            12 => module12::Module12InputData::mix_to_command(subcmd),
            _ => unreachable!(),
        }
    })
}

// ------------------------------------------------------------------------------------------------

fn show_list_of_available_problems() {
    let get_colored_string = |p_num| {
        if AVAILABLE_PROBLEMS.contains(&p_num) { cformat!("<g><s><u>Problem {}", p_num) }
        else { cformat!("<r><strike>Problem {}", p_num) }
    };
    for pnum in (1..=12).step_by(2) {
        let pnum2 = pnum + 1;
        cprintln!("{}\t{}", get_colored_string(pnum), get_colored_string(pnum2));
    }
}
