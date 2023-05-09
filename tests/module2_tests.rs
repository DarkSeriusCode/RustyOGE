extern crate rusty_oge;

use std::collections::HashMap;
use rusty_oge::module2::*;
use rusty_oge::utils::Validated;

// ----------------------------------------------------------------------------

const NUM_CODES: [(&str, &str); 6] = [("А", "01"), ("Д", "100"), ("К", "101"),
                                      ("Н", "10"), ("О", "111"), ("С", "000")];

const NUM_CODES2: [(&str, &str); 6] = [("А", "01"), ("В", "011"), ("Д", "100"),
                                       ("О", "111"), ("Р", "010"), ("У", "001")];

const RUS_CODES: [(&str, &str); 33] = [
    ("А", "1"), ("Б", "2"), ("В", "3"), ("Г", "4"), ("Д", "5"), ("Е", "6"), 
    ("Ё", "7"), ("Ж", "8"), ("З", "9"), ("И", "10"), ("Й", "11"), ("К", "12"),
    ("Л", "13"), ("М", "14"), ("Н", "15"), ("О", "16"), ("П", "17"), ("Р", "18"),
    ("С", "19"), ("Т", "20"), ("У", "21"), ("Ф", "22"), ("Х", "23"), ("Ц", "24"),
    ("Ч", "25"), ("Ш", "26"), ("Щ", "27"), ("Ъ", "28"), ("Ы", "29"), ("Ь", "30"),
    ("Э", "31"), ("Ю", "32"), ("Я", "33")
];

const ENG_CODES: [(&str, &str); 26] = [
    ("A", "1"), ("B", "2"), ("C", "3"), ("D", "4"), ("E", "5"), ("F", "6"),
    ("G", "7"), ("H", "8"), ("I", "9"), ("J", "10"), ("K", "11"), ("L", "12"),
    ("M", "13"), ("N", "14"), ("O", "15"), ("P", "16"), ("Q", "17"), ("R", "18"),
    ("S", "19"), ("T", "20"), ("U", "21"), ("V", "22"), ("W", "23"), ("X", "24"),
    ("Y", "25"), ("Z", "26")
];

// ------------------------------------------------------------------------------------------------

fn get_codes(raw_codes: Vec<(&str, &str)>) -> HashMap<String, String> {
    let mut map = HashMap::new();
 
    for (letter, code) in raw_codes {
        map.insert(code.to_string(), letter.to_string());
    }

    map
}

enum CodeType {
    Numbers,
    Numbers2,
    Russian,
    English,
    Custom(Vec<(&'static str, &'static str)>),
}

impl CodeType {
    fn get(&self) -> HashMap<String, String> {
        get_codes(match self {
            Self::Numbers => Vec::from(NUM_CODES),
            Self::Numbers2 => Vec::from(NUM_CODES2),
            Self::Russian => Vec::from(RUS_CODES),
            Self::English => Vec::from(ENG_CODES),
            Self::Custom(v) => v.clone(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

fn try_solve(input_data: InputData, right_answer: &str) {
    match solve(input_data) {
        Ok(answer) => assert_eq!(answer, right_answer),
        Err(err) => panic!("{}", err),
    }
}

fn make_input_data(codes: CodeType, strs: Vec<&str>, spec: ProblemSpec) -> InputData {
    let input_data = InputData {
        codes: codes.get(),
        encoded_strings: Vec::from_iter(strs.iter().map(|s| s.to_string())),
        spec,
    };
    assert!(input_data.is_valid());
    input_data
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem7() {
    let input = make_input_data(CodeType::Numbers, vec!["10111101", "1010110", "10111000"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "НОС");
}

#[test]
fn problem27() {
    let input = make_input_data(CodeType::Numbers, vec!["100101000", "101111100", "100111101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "КОД"); 
}

#[test]
fn problem47() {
    let input = make_input_data(CodeType::Numbers, vec!["1010110", "100000101", "00011110001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "СОДА");
}

#[test]
fn problem67() {
    let input = make_input_data(CodeType::Numbers, vec!["10111101", "00011110", "100111101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "СОН");
}

#[test]
fn problem87() {
    let input = make_input_data(CodeType::Numbers, vec!["100101000", "100000101", "0110001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АДА");
}

#[test]
fn problem107() {
    let input = make_input_data(CodeType::Numbers, vec!["10111101", "100111101", "0000110"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "САН");
}

#[test]
fn problem127() {
    let input = make_input_data(CodeType::Numbers, vec!["1010110", "11110001", "100000101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ОДА");
}

#[test]
fn problem227() {
    let codes = CodeType::Custom(vec![("А", "*-"), ("Г", "--*"), ("М", "--"),
                            ("К", "-*-"), ("Ю", "**--")]);
    let input = make_input_data(codes, vec!["--*-----***---*-*-"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МАМГЮКА");
}

#[test]
fn problem247() {
    let codes = CodeType::Custom(vec![("Н", "-*"), ("К", "-*-"), ("И", "**"),
                            ("Л", "*-**"), ("М", "--")]);
    let input = make_input_data(codes, vec!["-*-*-*--**-**-*-**"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ННКНЛКИ");
}

#[test]
fn problem267() {
    let codes = CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"),
                            ("Т", "-"), ("Ж", "***-")]);
    let input = make_input_data(codes, vec!["*--***-**--**-**-*--"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АДЛТДДТАТ");
}

#[test]
fn problem287() {
    let codes = CodeType::Custom(vec![("К", "+_+"), ("Л", "_*"), ("М", "*+"), 
                            ("Н", "_++"), ("О", "*"), ("П", "__+"),
                            ("Р", "__")]);
    let input = make_input_data(codes, vec!["*+_++_++___*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МННРЛ");
}

#[test]
fn problem327() {
    let input = make_input_data(CodeType::Russian, vec!["3135420", "2102030", "1331320", "2033510"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "БИТЬ");
}

#[test]
fn problem348() {
    let input = make_input_data(CodeType::Russian, vec!["20335", "21120", "31321", "51201"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ДАТА");
}

#[test]
fn problem368() {
    let input = make_input_data(CodeType::Russian, vec!["112233", "135793", "203014", "412030"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ГАТЬ");
}

#[test]
fn problem388() {
    let input = make_input_data(CodeType::Russian, vec!["1012", "1210", "1565", "5651"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ДЕДА");
}

#[test]
fn problem408() {
    let input = make_input_data(CodeType::Russian, vec!["8102030", "8112131", "8112233", "8152535"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ЖИТЬ");
}

#[test]
fn problem428() {
    let input = make_input_data(CodeType::Russian, vec!["3102030", "3102033", "3112030", "3112233"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВИТЬ");
}

#[test]
fn problem448() {
    let input = make_input_data(CodeType::English, vec!["2016", "2345", "4523", "6120"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "FAT");
}

#[test]
fn problem468() {
    let input = make_input_data(CodeType::English, vec!["1234", "2013", "3120", "4321"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "CAT");
}

#[test]
fn problem488() {
    let input = make_input_data(CodeType::English, vec!["18205", "20158", "20518", "81205"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "HATE");
}

#[test]
fn problem508() {
    let input = make_input_data(CodeType::English, vec!["17205", "20127", "20217", "71205"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "GATE");
}

#[test]
fn problem528() {
    let input = make_input_data(CodeType::English, vec!["121", "245", "913", "935"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ICE");
}

#[test]
fn problem548() {
    let codes = CodeType::Custom(vec![("Р", "C?"), ("Ы", "??C"), ("Б", "??"),
                            ("К", "?C"), ("А", "?C?")]);
    let input = make_input_data(codes, vec!["????C?C"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "БЫК");
}

#[test]
fn problem568() {
    let codes = CodeType::Custom(vec![("М", "C?"), ("Ы", "?CC"), ("Ш", "??"),
                            ("К", "?C"), ("А", "?C?")]);
    let input = make_input_data(codes, vec!["C??C??C"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МАК");
}

#[test]
fn problem588() {
    let codes = CodeType::Custom(vec![("Л", "?C"), ("Е", "???"), ("Н", "CC"),
                            ("К", "C?"), ("А", "CC?")]);
    let input = make_input_data(codes, vec!["?CCC?C?"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ЛАК");
}

#[test]
fn problem608() {
    let codes = CodeType::Custom(vec![("М", "?C"), ("И", "???"), ("Ш", "CC"),
                            ("К", "C?"), ("А", "CC?")]);
    let input = make_input_data(codes, vec!["?CCC?CC"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МАШ");
}

#[test]
fn problem628() {
    let codes = CodeType::Custom(vec![("Б", "110"), ("И", "01"), ("С", "100"),
                            ("Е", "10"), ("Р", "11")]);
    let input = make_input_data(codes, vec!["11010001100"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "БСИС");
}

#[test]
fn problem648() {
    let codes = CodeType::Custom(vec![("М", "01"), ("Е", "100"), ("Т", "110"), 
                            ("Л", "101"), ("А", "10")]);
    let input = make_input_data(codes, vec!["1101000110"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ТЕМА");
}

#[test]
fn problem668() {
    let codes = CodeType::Custom(vec![("А", "10"), ("Б", "110"), ("В", "12"), 
                            ("Г", "102"), ("Д", "0"), ("Е", "22"), ("Ж", "122")]);
    let input = make_input_data(codes, vec!["101212210102"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АВЖАГ")
}

#[test]
fn problem708() {
    let codes = CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Ж", "*-**"),
                            ("Л", "-"), ("Т", "***-")]);
    let input = make_input_data(codes, vec!["*--***-**--**-*--"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АДЖЛДЛАЛ");
}

#[test]
fn problem751() {
    let codes = CodeType::Custom(vec![("К", "!!?"), ("И", "!!"), ("С", "?!"),
                            ("Л", "???"), ("О", "?!")]);
    let input = make_input_data(codes, vec!["!!??!???"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "КОЛ");
}

#[test]
fn problem771() {
    let codes = CodeType::Custom(vec![("Р", "!!?"), ("Е", "!!"), ("Д", "!?"),
                            ("И", "???"), ("С", "?!")]);
    let input = make_input_data(codes, vec!["?!!!!?"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "СЕД");
}

#[test]
fn problem803() {
    let codes = CodeType::Custom(vec![("Ш", "01"), ("К", "11"), ("О", "100"),
                            ("Л", "101"), ("А", "10")]);
    let input = make_input_data(codes, vec!["1011011"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ЛАК");
}

#[test]
fn problem823() {
    let codes = CodeType::Custom(vec![("С", "110"), ("А", "01"), ("Д", "100"), 
                            ("И", "10"), ("К", "11")]);
    let input = make_input_data(codes, vec!["1011110"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ИКС");
}

#[test]
fn problem845() {
    let codes = CodeType::Custom(vec![("П", "@@@&"), ("Р", "@&&"), ("И", "&@"),
                            ("В", "&&@"), ("Е", "&&&@"), ("Т", "@&@")]);
    let input = make_input_data(codes, vec!["&&@&&&@@&@&&&@@&&"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВЕТЕР");
}

#[test]
fn problem865() {
    let codes = CodeType::Custom(vec![("В", "@@@"), ("О", "@&"), ("Л", "&@@"),
                            ("Г", "&@&"), ("А", "&&&")]);
    let input = make_input_data(codes, vec!["&@&@&&@@@&@@@&&&"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ГОЛОВА");
}

#[test]
fn problem886() {
    let codes = CodeType::Custom(vec![("С", "110"), ("М", "10"), ("А", "00"),
                            ("О", "001"), ("Р", "101"), ("К", "010")]);
    let input = make_input_data(codes, vec!["10001101110"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МОРС");
}

#[test]
fn problem906() {
    let codes = CodeType::Custom(vec![("С", "100"), ("М", "01"), ("А", "00"), 
                            ("О", "001"), ("Р", "101"), ("К", "010")]);
    let input = make_input_data(codes, vec!["101001010"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "РОК");
}

#[test]
fn problem926() {
    let codes = CodeType::Custom(vec![("Р", "CF"), ("Ы", "FFC"), ("В", "FF"), 
                            ("О", "FC"), ("С", "FCF")]);
    let input = make_input_data(codes, vec!["FFFCCFFCF"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВОРС");
}

#[test]
fn problem946() {
    let codes = CodeType::Custom(vec![("К", "CF"), ("О", "FFC"), ("В", "FF"),
                            ("Е", "FC"), ("Р", "FCF")]);
    let input = make_input_data(codes, vec!["FFFCCFFFC"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВЕКО");
}

#[test]
fn problem1018() {
    let codes = CodeType::Custom(vec![("П", "!!?"), ("И", "!!"), ("Р", "!?"),
                            ("А", "???"), ("Т", "?!")]);
    let input = make_input_data(codes, vec!["!?!!?!???"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "РИТА");
}

#[test]
fn problem1038() {
    let codes = CodeType::Custom(vec![("С", "!!?"), ("В", "!!"), ("И", "!?"),
                            ("Т", "???"), ("Е", "?!"), ("Р", "!!!")]);
    let input = make_input_data(codes, vec!["!!!?????!"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВИТЕ");
}

#[test]
fn problem1078() {
    let input = make_input_data(CodeType::Numbers2, vec!["0100100101", "011011111100", "0100110001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВВОД");
}

#[test]
fn problem1121() {
    let codes = CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                            ("Х", "****")]);
    let input = make_input_data(codes, vec!["**-***-*--*-****-"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "УЖАТАХТ");
}

#[test]
fn problem1161() {
    let codes = CodeType::Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                            ("Д", "-*"), ("Е", "+-+"), ("Ж", "**-")]);
    let input = make_input_data(codes, vec!["*+-++-**-**"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ГБДАДА");
}

#[test]
fn problem4548() {
    let input = make_input_data(CodeType::Russian, vec!["92610", "36910", "13131", "23456"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВЕЗИ");
}

#[test]
fn problem4585() {
    let input = make_input_data(CodeType::Numbers2, vec!["11101001", "010111011", "01001010"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "РОВ");
}

#[test]
fn problem4644() {
    let codes = CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                            ("Ч", "---*")]);
    let input = make_input_data(codes, vec!["*-**-*--*---**--*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АИНГЧАН");
}

#[test]
fn problem4717() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001001", "11101001", "10001010"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ДАР");
}

#[test]
fn problem4776() {
    let input = make_input_data(CodeType::Russian, vec!["31212", "12987", "10926", "36510"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВЕДИ");
}

#[test]
fn problem4893() {
    let input = make_input_data(CodeType::Numbers2, vec!["011111010", "01001001", "01001010"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВОР");
}

#[test]
fn problem4932() {
    let input = make_input_data(CodeType::Numbers2, vec!["0100100101", "010111100", "10011101001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "РОД");
}

#[test]
fn problem5124() {
    let codes = CodeType::Custom(vec![("М", "--"), ("Н", "-*"), ("С", "***"), ("У", "**-"),
                            ("А", "*-")]);
    let input = make_input_data(codes, vec!["*---**--*****--*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АМУНСАН");
}

#[test]
fn problem5264() {
    let codes = CodeType::Custom(vec![("К", "+-+"), ("Л", "-*"), ("М", "*+"), ("Н", "-++"),
                            ("О", "*"), ("П", "--+"), ("Р", "--")]);
    let input = make_input_data(codes, vec!["*+-++-++---*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "МННРЛ");
}

#[test]
fn problem5307() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001001", "0100100101", "111011111100"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ОВОД");
}

#[test]
fn problem5393() {
    let codes = CodeType::Custom(vec![("С", "***"), ("У", "**-"), ("А", "*-"), ("М", "--"),
                            ("Н", "-*")]);
    let input = make_input_data(codes, vec!["-*--*-**-*-***--"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "НМАУАСМ");
}

#[test]
fn problem5552() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001010", "0100110001", "01000110001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "РУДА");
}

#[test]
fn problem5655() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001010", "11110001", "0100100101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ОДА");
}

#[test]
fn problem5775() {
    let codes = CodeType::Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                            ("Е", "-*"), ("И", "+-+"), ("К", "**-")]);
    let input = make_input_data(codes, vec!["*+-++-**-**"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ГБЕАЕА");
}

#[test]
fn problem5800() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001010", "01111110001", "10011101001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВОДА")
}

#[test]
fn problem5886() {
    let input = make_input_data(CodeType::Numbers2, vec!["0110001", "0100110001", "10011101001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АДА");
}

#[test]
fn problem6260() {
    let input = make_input_data(CodeType::Numbers2, vec!["11101001", "100111", "0100100101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ДО");
}

#[test]
fn problem6421() {
    let input = make_input_data(CodeType::Numbers2, vec!["01001001", "100011111010", "10011101001"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ДВОР");
}

#[test]
fn problem12851() {
    let codes = CodeType::Custom(vec![("А", "01"), ("В", "10"), ("К", "000"), ("О", "111"),
                            ("Р", "0011"), ("Т", "1101")]);
    let input = make_input_data(codes, vec!["101110011111110101"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ВОРОТА");
}

#[test]
fn problem18170() {
    let codes = CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                            ("Ч", "---*")]);
    let input = make_input_data(codes, vec!["*-**-*--*---**--*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "АИНГЧАН");
}

#[test]
fn problem18185() {
    let codes = CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                            ("Ч", "---*")]);
    let input = make_input_data(codes, vec!["-**-**--**----*"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "НАИГАЧ");
}

#[test]
fn problem18423() {
    let codes = CodeType::Custom(vec![("А", "01"), ("Б", "100"), ("К", "101"), ("Л", "111"),
                            ("О", "00"), ("С", "110")]);
    let input = make_input_data(codes, vec!["001001110110100"],
                                ProblemSpec::new(true, false, OutputDataType::DecodedString));
    try_solve(input, "ОБЛАКО");
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem147() {
    let codes = CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                               ("П", "@~+"), ("О", "+"), ("И", "~")]);
    let input = make_input_data(codes, vec!["+~+~+@@~+"],
                                ProblemSpec::new(false, true, OutputDataType::DecodedString));
    try_solve(input, "ОЛИМП");
}

#[test]
fn problem167() {
    let codes = CodeType::Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"),
                               ("И", "@~*"), ("Т", "@*"), ("О", "~*")]);
    let input = make_input_data(codes, vec!["*@@~**~*~"],
                                ProblemSpec::new(false, true, OutputDataType::DecodedString));
    try_solve(input, "ЛИМОН");
}

#[test]
fn problem187() {
    let codes = CodeType::Custom(vec![("Ж", "+#"), ("Е", "+^#"), ("С", "#"),
                                ("А", "^"), ("К", "^#"), ("Л", "#+")]);
    let input = make_input_data(codes, vec!["#++^##^#^"],
                                ProblemSpec::new(false, true, OutputDataType::DecodedString));
    try_solve(input, "ЛЕСКА");
}

#[test]
fn problem207() {
    let codes = CodeType::Custom(vec![("А", "+#"), ("Е", "#+"), ("Л", "~"),
                                ("П", "#"), ("Т", "+~#"), ("О", "~#")]);
    let input = make_input_data(codes, vec!["#~#~#++~#"],
                                ProblemSpec::new(false, true, OutputDataType::DecodedString));
    try_solve(input, "ПОЛЕТ");
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem1101() {
    let codes = CodeType::Custom(vec![("Ж", "+#"), ("З", "+^#"), ("И", "#"), ("Й", "^"),
                            ("К", "^#"), ("Л", "#+")]);
    let input = make_input_data(codes, vec!["#++^##^#^"],
                                ProblemSpec::new(false, true, OutputDataType::Length));
    try_solve(input, "5");
}

#[test]
fn problem1240() {
    let codes = CodeType::Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"), ("И", "@~*"),
                            ("Т", "@*"), ("О", "~*")]);
    let input = make_input_data(codes, vec!["*@@~**~*~"],
                                ProblemSpec::new(false, true, OutputDataType::Length));
    try_solve(input, "5");
}

#[test]
fn problem6221() {
    let codes = CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"), ("Н", "@~+"),
                            ("О", "+"), ("П", "~")]);
    let input = make_input_data(codes, vec!["+~+~+@@~+"],
                                ProblemSpec::new(false, true, OutputDataType::Length));
    try_solve(input, "5");
}

#[test]
fn problem18256() {
    let codes = CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"), ("Н", "@~+"),
                            ("О", "+"), ("П", "~")]);
    let input = make_input_data(codes, vec!["+~+~@~+"],
                                ProblemSpec::new(false, true, OutputDataType::Length));
    try_solve(input, "4");
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem1141() {
    let codes = CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"), ("Т", "-"),
                            ("Ж", "***-")]);
    let input = make_input_data(codes, vec!["*--***-**--**-*--"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "8");
}

#[test]
fn problem4565() {
    let codes = CodeType::Custom(vec![("Е", "*"), ("Н", "-*"), ("О", "---"), ("З", "--**"),
                            ("Щ", "--*-")]);
    let input = make_input_data(codes, vec!["-***---*"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "5");
}

#[test]
fn problem4689() {
    let codes = CodeType::Custom(vec![("А", "*-"), ("Г", "--*"), ("И", "**"), ("П", "*--*"),
                            ("М", "--")]);
    let input = make_input_data(codes, vec!["*-*--*--**-**--*"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "6");
}

#[test]
fn problem4742() {
    let codes = CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                            ("Х", "****")]);
    let input = make_input_data(codes, vec!["***-**-***-*-**-"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "5");
}

#[test]
fn problem5311() {
    let codes = CodeType::Custom(vec![("А", "~"), ("Б", "o++"), ("В", "oo+"), ("Г", "~+"),
                            ("Д", "o~"), ("Е", "+o+"), ("Ё", "~~o")]);
    let input = make_input_data(codes, vec!["~+o++o~~o~~"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "6");
}

#[test]
fn problem6197() {
    let codes = CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                            ("Х", "****")]);
    let input = make_input_data(codes, vec!["-*-*-**--*--"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "7");
}

#[test]
fn problem6354() {
    let codes = CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                            ("Х", "****")]);
    let input = make_input_data(codes, vec!["**-*-***-*--**-"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "6");
}

#[test]
fn problem18211() {
    let codes = CodeType::Custom(vec![("Е", "0"), ("Н", "10"), ("О", "111"), ("З", "1100"),
                            ("Щ", "1101")]);
    let input = make_input_data(codes, vec!["11110010011000"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "7");
}

#[test]
fn problem18226() {
    let codes = CodeType::Custom(vec![("А", "01"), ("Г", "110"), ("И", "00"), ("П", "0110"),
                            ("М", "11")]);
    let input = make_input_data(codes, vec!["0101101100100110"],
                                ProblemSpec::new(true, false, OutputDataType::Length));
    try_solve(input, "6");
}

// ------------------------------------------------------------------------------------------------

#[test]
fn problem4835() {
    let codes = CodeType::Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                            ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]);
    let input = make_input_data(codes, vec!["...o..o.oo...o..oooo.o.oo"],
                                ProblemSpec::new(true, false, OutputDataType::RepeatingChars));
    try_solve(input, "ДЕ");
}

#[test]
fn problem5755() {
    let codes = CodeType::Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                            ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]);
    let input = make_input_data(codes, vec![".o..o.o.oo.o..o..o....o.."],
                                ProblemSpec::new(true, false, OutputDataType::RepeatingChars));
    try_solve(input, "АБ");
}

