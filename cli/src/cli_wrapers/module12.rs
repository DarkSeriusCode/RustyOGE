use std::path::PathBuf;
use std::ffi::OsString;

use regex::Regex;
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
        data_size::{DataSize, DataSizeUnit},
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
    let re = Regex::new(r"(?<num>\d+)(?<unit>B|Kb|Mb)").expect("Cannot create Regex!");
    let Some(capture) = re.captures(s) else {
        return Err(Error::raw(ErrorKind::Format,
                              format!("Invalid format \"{}\". Format as <num><unit>", s)));
    };
    // TODO: Убрать else, т.к он не выполнится никогда
    let Ok(num): Result<usize, _> = capture["num"].parse() else {
        return Err(Error::raw(ErrorKind::InvalidValue,
                              format!("\"{}\" in \"{}\" should be a number", &capture["num"], s)));
    };

    let unit = match &capture["unit"] {
        "B"  => DataSizeUnit::Bytes,
        "Kb" => DataSizeUnit::Kb,
        "Gb" => DataSizeUnit::Mb,
        unit => return Err(Error::raw(ErrorKind::InvalidValue,
                                      format!("Unknown data unit {unit}"))),
    };

    Ok(DataSize::new(num, unit))
}
