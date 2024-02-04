use clap::{
    value_parser,
    Command, Arg, ArgGroup, ArgMatches,
    FromArgMatches, ValueEnum,
    error::{Error, ErrorKind},
};

use rusty_oge::module1::{InputData, ProblemSpec, InputText};
use rusty_oge::utils::{Validated, data_size::DataSizeUnit};

use crate::utils::CommandArgMixin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[repr(u32)]
enum CLIDataSizeUnit {
    Bytes,
    Kb,
    Mb,
}

impl From<CLIDataSizeUnit> for DataSizeUnit {
    fn from(value: CLIDataSizeUnit) -> Self {
        match value {
            CLIDataSizeUnit::Bytes => Self::Bytes,
            CLIDataSizeUnit::Kb    => Self::Kb,
            CLIDataSizeUnit::Mb    => Self::Mb,
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Module1InputData(pub InputData);

impl Module1InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("bits_in_char")
                .long("bits_in_char")
                .short('b')
                .required(true)
                .value_parser(value_parser!(usize))
                .help("How many bits are needed to encode 1 char"),

            Arg::new("concrete_text")
                .long("concrete_text")
                .short('c')
                .conflicts_with("text_info")
                .help("Given text in the problem. Note: you CANNOT use --concrete_text and \
                      --pages, --lines, --chars together!"),

            Arg::new("pages")
                .long("pages")
                .group("text_info")
                .required(true)
                .value_parser(value_parser!(usize))
                .help("How many \"pages\" given in the problem"),
            Arg::new("lines")
                .long("lines")
                .group("text_info")
                .required(true)
                .value_parser(value_parser!(usize))
                .help("How many \"lines\" given in the problem"),
            Arg::new("chars")
                .long("chars")
                .group("text_info")
                .required(true)
                .value_parser(value_parser!(usize))
                .help("How many \"chars\" given in the problem"),

            Arg::new("find_word")
                .long("find_word")
                .group("spec")
                .value_name("bytes")
                .value_parser(value_parser!(usize))
                .help("In the problem, you need to find a word that increases/decreases the \
                       text size by <bytes> bytes"),
            Arg::new("calc_text_size")
                .long("calc_text_size")
                .group("spec")
                .value_name("data unit")
                .value_parser(value_parser!(CLIDataSizeUnit))
                .help("In the problem, you need to calc the text size in <data unit>")
        ]
    }

    pub fn get_groups() -> Vec<ArgGroup> {
        vec![
            ArgGroup::new("text_info")
                .multiple(true),
            ArgGroup::new("spec")
                .required(true),
        ]
    }
}

impl CommandArgMixin for Module1InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args()).groups(Self::get_groups())
    }
}

impl FromArgMatches for Module1InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let bits_in_char = *matches.get_one::<usize>("bits_in_char").unwrap();
        let text: InputText;
        let spec: ProblemSpec;

        if let Some(concrete_text) = matches.get_one::<String>("concrete_text") {
            text = InputText::ConcreteText(concrete_text.clone());
        } else {
            text = InputText::TextInfo {
                pages: *matches.get_one::<usize>("pages").unwrap(),
                lines: *matches.get_one::<usize>("lines").unwrap(),
                chars: *matches.get_one::<usize>("chars").unwrap(),
            };
        }

        // Если установлен --find_word
        if let Some(bytes) = matches.get_one::<usize>("find_word") {
            spec = ProblemSpec::FindWord(*bytes);
        }
        // Если установлен --calc_text_size
        else if let Some(data_unit) = matches.get_one::<CLIDataSizeUnit>("calc_text_size") {
            spec = ProblemSpec::CalcTextSize(data_unit.to_owned().into())
        } else {
            unreachable!()
        }

        let input_data = InputData::new(bits_in_char, text, spec);
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
