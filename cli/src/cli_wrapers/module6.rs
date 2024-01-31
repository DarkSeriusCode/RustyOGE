use std::path::PathBuf;

use clap::{
    value_parser,
    Command, Arg, ArgMatches,
    FromArgMatches,
    error::{Error, ErrorKind},
};

use rusty_oge::module6::{InputData, ProblemSpec};
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone)]
pub struct Module6InputData(pub InputData);

impl Module6InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("file_name")
                .long("file_name")
                .short('f')
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .help("Path to a program file to execute"),
            Arg::new("program_input")
                .long("program_input")
                .short('i')
                .required(true)
                .value_parser(value_parser!(String))
                .help("A string contains input data for a program"),
            Arg::new("expected_output")
                .long("expected_output")
                .short('o')
                .required(true)
                .value_parser(value_parser!(String))
                .help("What program should print (return)"),
        ]
    }
}

impl CommandArgMixin for Module6InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module6InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let file_name = matches.get_one::<PathBuf>("file_name").unwrap();
        let program_input = matches.get_one::<String>("program_input").unwrap();
        let expected_output = matches.get_one::<String>("expected_output").unwrap();

        let spec = ProblemSpec::new(expected_output.to_string());
        let input_data = InputData::new(file_name, program_input, spec);
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
