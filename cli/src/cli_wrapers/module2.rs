use clap::{
    value_parser,
    Command, Arg, ArgGroup, ArgMatches, ArgAction,
    FromArgMatches, ValueEnum,
    error::{Error, ErrorKind}
};

use rusty_oge::module2::{InputData, ProblemSpec, OutputDataType, Codes};
use rusty_oge::utils::Validated;

use crate::utils::CommandArgMixin;

const RUS_ALPHABET: &[(&str, &str)] = &[
    ("А", "1"), ("Б", "2"), ("В", "3"), ("Г", "4"), ("Д", "5"), ("Е", "6"), 
    ("Ё", "7"), ("Ж", "8"), ("З", "9"), ("И", "10"), ("Й", "11"), ("К", "12"),
    ("Л", "13"), ("М", "14"), ("Н", "15"), ("О", "16"), ("П", "17"), ("Р", "18"),
    ("С", "19"), ("Т", "20"), ("У", "21"), ("Ф", "22"), ("Х", "23"), ("Ц", "24"),
    ("Ч", "25"), ("Ш", "26"), ("Щ", "27"), ("Ъ", "28"), ("Ы", "29"), ("Ь", "30"),
    ("Э", "31"), ("Ю", "32"), ("Я", "33")
];

const ENG_ALPHABET: &[(&str, &str)] = &[
    ("A", "1"), ("B", "2"), ("C", "3"), ("D", "4"), ("E", "5"), ("F", "6"),
    ("G", "7"), ("H", "8"), ("I", "9"), ("J", "10"), ("K", "11"), ("L", "12"),
    ("M", "13"), ("N", "14"), ("O", "15"), ("P", "16"), ("Q", "17"), ("R", "18"),
    ("S", "19"), ("T", "20"), ("U", "21"), ("V", "22"), ("W", "23"), ("X", "24"),
    ("Y", "25"), ("Z", "26")
];

// ------------------------------------------------------------------------------------------------

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
enum Alphabet {
    Russian,
    English,
}

impl Into<Vec<(String, String)>> for Alphabet {
    fn into(self) -> Vec<(String, String)> {
        match self {
            Self::Russian => &RUS_ALPHABET,
            Self::English => &ENG_ALPHABET,
        }.iter().map(|(l, c)| (l.to_string(), c.to_string())).collect()
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
enum CLIOutputDataType {
    DecodedString,
    Length,
    RepeatingChars,
}

impl Into<OutputDataType> for CLIOutputDataType {
    fn into(self) -> OutputDataType {
        match self {
            Self::DecodedString  => OutputDataType::DecodedString,
            Self::Length         => OutputDataType::Length,
            Self::RepeatingChars => OutputDataType::RepeatingChars,
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Module2InputData(pub InputData);

impl Module2InputData {
    pub fn get_args() -> Vec<Arg> {
        vec![
            Arg::new("codes")
                .long("codes")
                .short('c')
                .group("codes_source")
                .num_args(1..)
                .value_parser(parse_codes)
                .help("Codes given in the probelm. Format <letter>=<code>"),
            Arg::new("alphabet")
                .long("alphabet")
                .short('a')
                .group("codes_source")
                .value_parser(value_parser!(Alphabet))
                .help("Pre-defined alphabet codes"),
            Arg::new("encoded_strings")
                .long("strings")
                .short('s')
                .required(true)
                .num_args(1..)
                .help("Given strings in the problem"),
            Arg::new("one_decoding")
                .long("one_decoding")
                .action(ArgAction::SetTrue)
                .help("Set if you need to find ONLY ONE string that can be successfully decoded"),
            Arg::new("unique_chars")
                .long("unique_chars")
                .action(ArgAction::SetTrue)
                .help("Set if the decoded string MUST contain only unique characters"),
            Arg::new("output_type")
                .long("output")
                .short('o')
                .required(true)
                .value_parser(value_parser!(CLIOutputDataType))
                .help("What information about the decoded string should be output")
        ]
    }

    pub fn get_groups() -> Vec<ArgGroup> {
        vec![
            ArgGroup::new("codes_source")
                .required(true)
        ]
    }
}

impl CommandArgMixin for Module2InputData {
    fn mix_to_command(cmd: Command) -> Command {
        cmd.args(Self::get_args()).groups(Self::get_groups())
    }
}

impl FromArgMatches for Module2InputData {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        // Получаем codes
        let raw_codes: Vec<(String, String)>;
        if let Some(&alphabet) = matches.get_one::<Alphabet>("alphabet") {
            raw_codes = alphabet.into();
        } else {
            raw_codes = matches.get_many::<(String, String)>("codes")
                .unwrap_or_default()
                .map(|(f, s)| (f.to_string(), s.to_string()))
                .collect();
        }
        let codes: Codes = Codes::from_iter(raw_codes.iter().map(|pair| pair.to_owned()));

        // Получаем encoded_strings
        let encoded_strings: Vec<String> = matches.get_many::<String>("encoded_strings")
            .unwrap()
            .map(|s| s.to_string())
            .collect();

        // Получаем spec
        let one_decoding = *matches.get_one::<bool>("one_decoding").unwrap();
        let unique_chars = *matches.get_one::<bool>("unique_chars").unwrap();
        let output_type = *matches.get_one::<CLIOutputDataType>("output_type").unwrap();
        let spec = ProblemSpec::new(one_decoding, unique_chars, output_type.into());

        let input_data = InputData::new(codes, encoded_strings, spec);
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

fn parse_codes(s: &str) -> Result<(String, String), Error> {
    let Some((letter, code)) = s.split_once("=") else {
        return Err(Error::raw(ErrorKind::Format,
                              format!("Invalid format \"{}\". Format as <letter>=<code>", s)))
    };

    Ok((code.trim().to_string(), letter.trim().to_string()))
}
