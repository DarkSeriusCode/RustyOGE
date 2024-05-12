use std::path::PathBuf;
use std::ffi::OsString;

use clap::{
    value_parser,
    Command, Arg, ArgMatches,
    FromArgMatches,
    error::{Error, ErrorKind}
};

use rusty_oge::{
    module12::InputData,
    utils::{
        Validated,
        data_size::DataSize,
    },
};

use crate::utils::CommandArgMixin;

pub struct Module12InputData(pub InputData);

impl Module12InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("archive_path")
                .long("archive_path")
                .short('a')
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .help("A path to a given archive."),
            Arg::new("search_dir")
                .long("search_dir")
                .short('s')
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .help("In which directory in the archive search for files"),
            Arg::new("exts")
                .long("exts")
                .short('e')
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(OsString))
                .help("Find files with these extensions\nExample: --exts doc docx"),
            Arg::new("file_size")
                .long("minimum_file_size")
                .short('m')
                .value_parser(parse_data_size)
                .help("Minimum file size")
        ]
    }
}

impl CommandArgMixin for Module12InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args())
    }
}

impl FromArgMatches for Module12InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let archive_path = matches.get_one::<PathBuf>("archive_path").unwrap();
        let search_dir = matches.get_one::<PathBuf>("search_dir").unwrap();
        let exts: Vec<OsString> = matches.get_many::<OsString>("exts")
            .unwrap()
            .map(|s| s.to_owned())
            .collect();
        let file_size = matches.get_one::<DataSize>("file_size");

        let input_data = InputData::new(archive_path, search_dir, exts, file_size.copied());
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

fn parse_data_size(s: &str) -> Result<DataSize, Error> {
    s.parse::<DataSize>().map_err(|e| Error::raw(ErrorKind::InvalidValue, e.to_string()))
}
