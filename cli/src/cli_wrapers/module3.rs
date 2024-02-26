use clap::{
    value_parser,
    Command, Arg, ArgMatches,
    FromArgMatches, ValueEnum,
    error::{Error, ErrorKind},
};

use rusty_oge::module3::InputData;
use rusty_oge::utils::{NumberToFind, Validated};

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CLINumberToFind {
    Min,
    Max,
}

impl From<CLINumberToFind> for NumberToFind {
    fn from(value: CLINumberToFind) -> Self {
        match value {
            CLINumberToFind::Min => Self::Min,
            CLINumberToFind::Max => Self::Max,
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Module3InputData(pub InputData);

impl Module3InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("number_to_find")
                .short('n')
                .long("number_to_find")
                .required(true)
                .value_parser(value_parser!(CLINumberToFind))
                .help("PLACEHOLDER"),
            Arg::new("digits_in_number")
                .short('d')
                .long("digits_in_number")
                .value_parser(value_parser!(usize))
                .help("PLACEHOLDER"),
            Arg::new("expr")
                .short('e')
                .long("expression")
                .required(true)
                .help("PLACEHOLDER"),
            Arg::new("expr_res")
                .short('r')
                .long("expression_result")
                .required(true)
                .value_parser(value_parser!(bool))
                .help("PLACEHOLDER"),
        ]
    }
}

impl CommandArgMixin for Module3InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module3InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let number_to_find   = *matches.get_one::<CLINumberToFind>("number_to_find").unwrap();
        let digits_in_number = matches.get_one::<usize>("digits_in_number").copied();
        let expr             = matches.get_one::<String>("expr").unwrap();
        let expr_res         = *matches.get_one::<bool>("expr_res").unwrap();

        let input_data = InputData::new(number_to_find.into(), digits_in_number, &expr, expr_res);
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
