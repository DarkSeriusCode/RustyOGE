use std::convert::Into;

use clap::{
    value_parser,
    Command, Arg, ArgGroup, ArgMatches,
    FromArgMatches, ValueEnum,
    builder::ValueParser,
    error::{Error, ErrorKind},
};

use rusty_oge::module10::{Number, NumberToFind, InputData, ProblemSpec};
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CLINumberToFind {
    Min,
    Max,
}

impl Into<NumberToFind> for CLINumberToFind {
    fn into(self) -> NumberToFind {
        match self {
            Self::Min => NumberToFind::Min,
            Self::Max => NumberToFind::Max,
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Module10InputData(pub InputData);

impl Module10InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("numbers")
                .long("numbers")
                .short('n')
                .required(true)
                .num_args(1..)
                .value_parser(ValueParser::new(parse_number))
                .help("Numbers given in problem. Format: <number>x<base>,<number>x<base>..."),

            Arg::new("number_to_find")
                .long("number_to_find")
                .value_parser(value_parser!(CLINumberToFind))
                .help("Which number need to find: min or max"),

            Arg::new("find_num")
                .long("find_num")
                .group("spec")
                .value_parser(value_parser!(CLINumberToFind))
                .help("In the problem, you need to find min/max number in decimal system"),
            Arg::new("convert")
                .long("convert")
                .group("spec")
                .value_parser(value_parser!(u32).range(2..=36))
                .value_name("base")
                .help("In the problem, you need to convert a number into a different system"),
            Arg::new("find_digit_sum")
                .long("find_digit_sum")
                .group("spec")
                .value_name("base")
                .requires("number_to_find")
                .value_parser(value_parser!(u32).range(2..=36))
                .help("In the problem, you need to find a number that has max/min sum of its digits"),
            Arg::new("find_ones_count")
                .long("find_ones_count")
                .group("spec")
                .value_name("min or max")
                .value_parser(ValueParser::new(parse_number_to_find))
                .help("In the problem, you need to find a number that has min/max number of zeros \
                      in binary system")
        ]
    }

    pub fn get_groups() -> Vec<ArgGroup> {
        vec![
            ArgGroup::new("spec")
                .required(true),
        ]
    }
}

impl FromArgMatches for Module10InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let numbers = matches.get_many::<Number>("numbers").unwrap()
            .map(|i| i.to_owned()).collect::<Vec<_>>();

        let spec: ProblemSpec;
        // Если установлен --find_num
        if let Some(number_to_find) = matches.get_one::<CLINumberToFind>("find_num") {
            spec = ProblemSpec::FindNum(number_to_find.to_owned().into());
        }
        // Если установлен --convert
        else if let Some(&convert_base) = matches.get_one::<u32>("convert") {
            spec = ProblemSpec::Convert(convert_base);
        }
        // Если установлен --find_digit_sum
        else if let Some(&base) = matches.get_one::<u32>("find_digit_sum") {
            let number_to_find = matches.get_one::<CLINumberToFind>("number_to_find").unwrap();
            spec = ProblemSpec::FindDigitsSum(base, number_to_find.to_owned().into());
        }
        // Если установлен --find_ones_count
        else if let Some(number_to_find) = matches.get_one::<NumberToFind>("find_ones_count") {
            spec = ProblemSpec::FindOnesCount(*number_to_find);
        }
        else {
            unreachable!();
        }

        let input_data = InputData::new(numbers, spec);
        if let Err(e) = input_data.valid() {
            return Err(Error::raw(ErrorKind::InvalidValue, e));
        }
        Ok(Self(input_data))
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl CommandArgMixin for Module10InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args()).groups(Self::get_groups())
    }
}

// ------------------------------------------------------------------------------------------------

fn parse_number(s: &str) -> Result<Number, Error> {
    // Ошибки формата
    let Some((num, base)) = s.split_once('x') else {
        return Err(Error::raw(ErrorKind::Format,
                              format!("\"{}\" invalid format! Format as <number>x<base>", s)));
    };

    // base - не число
    let Ok(base) = base.parse::<u32>() else {
        return Err(Error::raw(ErrorKind::InvalidValue, format!("{} isn't a number!", base)));
    };

    // Ошибка создания Number
    match Number::new(num, base) {
        Ok(number) => Ok(number),
        Err(num_err) => Err(Error::raw(ErrorKind::InvalidValue, num_err.to_string())),
    }
}

fn parse_number_to_find(s: &str) -> Result<NumberToFind, Error> {
    match s {
        "min" => Ok(NumberToFind::Min),
        "max" => Ok(NumberToFind::Max),
        other => Err(Error::raw(ErrorKind::InvalidValue,
                                format!("Its values are min or max, not {other}"))),
    }
}
