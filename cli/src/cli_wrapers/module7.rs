use std::collections::HashMap;

use clap::{
    Command, Arg, ArgMatches,
    FromArgMatches,
    error::{Error, ErrorKind},
};

use rusty_oge::module7::InputData;
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone)]
pub struct Module7InputData(pub InputData);

impl Module7InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("ip_parts")
                .long("ip_parts")
                .short('i')
                .required(true)
                .num_args(1..)
                .value_parser(parse_ip_parts)
                .help("Given in the probelm parts of an IP address. Format <part name>=<ip part>\n\
                       Example: А=.33 Б=3.232"),
        ]
    }
}

impl CommandArgMixin for Module7InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module7InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let ip_parts: HashMap<char, String> = matches.get_many::<(char, String)>("ip_parts")
            .unwrap_or_default()
            .map(|(c, s)| (c.to_owned(), s.to_string()))
            .collect();

        let input_data = InputData::new(ip_parts);
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

fn parse_ip_parts(s: &str) -> Result<(char, String), Error> {
    let Some((part_name, ip_part)) = s.split_once('=') else {
        return Err(Error::raw(ErrorKind::Format,
                              format!("Invalid format \"{}\". Format as <name>=<ip_part>", s)))
    };
    if part_name.chars().count() != 1 {
        return Err(Error::raw(ErrorKind::InvalidValue, format!("The name of the IP part should be \
                                                               a single char! Not {}", part_name)));
    }

    Ok((part_name.chars().next().unwrap(), ip_part.to_string()))
}
