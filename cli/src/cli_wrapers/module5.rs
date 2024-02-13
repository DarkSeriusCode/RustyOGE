use std::str::FromStr;

use clap::{
    value_parser,
    Command, Arg, ArgGroup, ArgMatches,
    FromArgMatches,
    error::{Error, ErrorKind},
};

use rusty_oge::module5::{self, InputData, ProblemSpec, CommandTable};
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone)]
pub struct Module5InputData(pub InputData);

impl Module5InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("commands")
                .long("commands")
                .short('c')
                .required(true)
                .num_args(1..)
                .value_parser(parse_command)
                .help("Given commands. Format <name>=<action>.\n<name> is just an one-char name.\n\
                      <action> has this format: <+|-|*|/|^|><number|one-char variable name>.\n\
                      Example: \"*5\", \"-b\", \"^2\", \"/b\""),
            Arg::new("begin_num")
                .long("begin_num")
                .short('b')
                .required(true)
                .value_parser(value_parser!(i32))
                .help("A beginning number given in the problem."),
            Arg::new("result_num")
                .long("result_num")
                .short('r')
                .required(true)
                .value_parser(value_parser!(i32))
                .help("What number should be after program execution"),

            Arg::new("program")
                .long("program")
                .group("spec")
                .help("Program, given in the problem"),
            Arg::new("algorithm_length")
                .long("algo_len")
                .short('a')
                .group("spec")
                .value_parser(value_parser!(usize))
                .help("Required algorithm length"),
        ]
    }

    pub fn get_groups() -> Vec<ArgGroup> {
        vec![
            ArgGroup::new("spec")
                .required(true)
        ]
    }
}

impl CommandArgMixin for Module5InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args()).groups(Self::get_groups())
    }
}

impl FromArgMatches for Module5InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let commands: CommandTable = matches.get_many::<(char, module5::Command)>("commands")
            .unwrap_or_default()
            .map(|i| i.to_owned())
            .collect();
        let begin_num = *matches.get_one::<i32>("begin_num").unwrap();
        let result_num = *matches.get_one::<i32>("result_num").unwrap();

        let spec = if let Some(program) = matches.get_one::<String>("program") {
            ProblemSpec::FindVariableValue(program.to_string())
        } else if let Some(algo_len) = matches.get_one::<usize>("algo_len") {
            ProblemSpec::MakeAlgorithm(*algo_len)
        } else {
            unreachable!()
        };

        let input_data = InputData::new(commands, begin_num, result_num, spec);
        if let Err(e) = input_data.valid() {
            return Err(Error::raw(ErrorKind::InvalidValue, e));
        }
        Ok(Self(input_data))
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

fn parse_command(s: &str) -> Result<(char, module5::Command), Error> {
    let Some((cmd_name, cmd_str)) = s.split_once('=') else {
        return Err(Error::raw(ErrorKind::Format,
                              format!("Invalid format \"{}\". Format as <name>=<action>", s)))
    };
    if cmd_name.chars().count() != 1 {
        return Err(Error::raw(ErrorKind::InvalidValue, format!("The name of the command should be \
                                                               a single char! Not {}", cmd_name)));
    }
    let cmd = match module5::Command::from_str(cmd_str) {
        Ok(cmd) => cmd,
        Err(e)  => return Err(Error::raw(ErrorKind::Format, e)),
    };

    Ok((cmd_name.chars().next().unwrap(), cmd))
}
