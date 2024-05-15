use std::collections::HashMap;

use clap::{
    Command, Arg, ArgMatches,
    FromArgMatches,
    error::{Error, ErrorKind},
};

use rusty_oge::module9::InputData;
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone)]
pub struct Module9InputData(pub InputData);

impl Module9InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("map")
                .long("map")
                .required(true)
                .num_args(1..)
                .value_parser(parse_map_parts)
                .help("Graph given in the problem. Format: <node_name>=<child1>,\
                      <child2>,...,<childN>\n\
                      Example: A=B,G  B=D,K,V  V=K  G=V,K,E  D=K  E=K\n\
                      Taken from https://inf-oge.sdamgia.ru/problem?id=18039"),
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
            Arg::new("exclude")
                .long("exclude")
                .num_args(1..)
                .help("The way must ignore these nodes"),
        ]
    }
}

impl CommandArgMixin for Module9InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module9InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let map_parts = matches.get_many::<(String, Vec<String>)>("map").unwrap()
                               .map(|i| i.to_owned()).collect::<Vec<_>>();
        let from         = matches.get_one::<String>("from").unwrap();
        let to           = matches.get_one::<String>("to").unwrap();
        let include      = matches.get_many::<String>("include").unwrap_or_default()
                               .map(|i| i.to_owned()).collect::<Vec<_>>();
        let exclude      = matches.get_many::<String>("exclude").unwrap_or_default()
                               .map(|i| i.to_owned()).collect::<Vec<_>>();

        let map = make_map_from_parts(&map_parts);

        let input_data = InputData::new(map, (from, to), include, exclude);
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

// ------------------------------------------------------------------------------------------------

const FORMAT_ERROR: &'static str = "Invalid format! Format as <node_name>=<child1>,\
<child2>,...,<childN>";

fn parse_map_parts(s: &str) -> Result<(String, Vec<String>), Error> {
    let Some((node_name, childern_str)) = s.split_once('=')
        else { return Err(Error::raw(ErrorKind::Format, FORMAT_ERROR)); };
    let children: Vec<String> = Vec::from_iter(childern_str.split(',').map(|s| s.trim().to_string()));

    Ok((node_name.trim().to_string(), children))
}

// ------------------------------------------------------------------------------------------------

fn make_map_from_parts(parts: &[(String, Vec<String>)]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for (key, value) in parts {
        map.insert(key.to_string(), value.clone());
        for child in value {
            map.entry(child.to_string()).or_default();
        }
    }
    map
}
