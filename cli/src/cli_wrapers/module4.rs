use std::collections::HashMap;

use regex::Regex;
use clap::{
    value_parser,
    Command, Arg, ArgMatches,
    FromArgMatches, ValueEnum,
    error::{Error, ErrorKind},
};

use rusty_oge::module4::{InputData, PathToFind};
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CLIPathToFind {
    Shortest,
    Longest,
}

impl From<CLIPathToFind> for PathToFind {
    fn from(value: CLIPathToFind) -> Self {
        match value {
            CLIPathToFind::Longest  => PathToFind::Longest,
            CLIPathToFind::Shortest => PathToFind::Shortest,
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Module4InputData(pub InputData);

impl Module4InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("map")
                .long("map")
                .required(true)
                .num_args(1..)
                .value_parser(parse_map_parts)
                .help("Table given in the problem. Format: <from>~<to>=<path len>\n\
                      Example: A~B=1 B~C=2 B~D=2 B~E=7 C~E=3 D~E=4\
                      (taken from https://inf-oge.sdamgia.ru/problem?id=3)\n\
                      Note: you don't need to write A~B=1 and B~A=1 at the same time!"),
            Arg::new("from")
                .long("from")
                .required(true)
                .help("Name of the start node"),
            Arg::new("to")
                .long("to")
                .required(true)
                .help("Name of the finish node"),
            Arg::new("include")
                .long("include")
                .num_args(1..)
                .help("The way must go through these nodes"),
            Arg::new("path_to_find")
                .long("path_to_find")
                .required(true)
                .value_parser(value_parser!(CLIPathToFind))
                .help("Specifies which path to find"),
        ]
    }
}

impl CommandArgMixin for Module4InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module4InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let map_parts    = matches.get_many::<(String, String, usize)>("map").unwrap()
                               .map(|i| i.to_owned()).collect::<Vec<_>>();
        let from         = matches.get_one::<String>("from").unwrap();
        let to           = matches.get_one::<String>("to").unwrap();
        let include      = matches.get_many::<String>("include").unwrap_or_default()
                               .map(|i| i.to_owned()).collect::<Vec<_>>();
        let path_to_find = *matches.get_one::<CLIPathToFind>("path_to_find").unwrap();

        let map = make_map_from_parts(&map_parts);

        let input_data = InputData::new(map, (from, to), include, path_to_find.into());
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

fn parse_map_parts(s: &str) -> Result<(String, String, usize), Error> {
    let re = Regex::new(r"(?<from>\w+)~(?<to>\w+)=(?<len>\d+)")
        .expect("Cannot create Regex!");
    let Some(capture) = re.captures(s) else {
        return Err(Error::raw(ErrorKind::Format,
                  format!("Invalid format \"{}\". Format as <from>~<to>=<path len>", s)));
    };
    let from = capture["from"].to_string();
    let to   = capture["to"].to_string();
    let len  = capture["len"].parse::<usize>().unwrap();

    Ok((from, to, len))
}

// ------------------------------------------------------------------------------------------------

fn make_map_from_parts(parts: &[(String, String, usize)]) -> HashMap<String, Vec<(String, usize)>> {
    let mut map: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (from, to, path_len) in parts {
        map.entry(from.into()).or_default().push((to.into(), *path_len));
        map.entry(to.into()).or_default().push((from.into(), *path_len));
    }
    map
}
