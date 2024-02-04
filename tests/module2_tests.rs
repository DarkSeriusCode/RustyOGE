extern crate rusty_oge;

use std::collections::HashMap;
use rusty_oge::module2::*;

#[macro_use]
mod test_macros;

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

Test! {
    Name = problem7,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["10111101", "1010110", "10111000"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "НОС"
}

Test! {
    Name = problem27,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["100101000", "101111100", "100111101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "КОД"
}

Test! {
    Name = problem47,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["1010110", "100000101", "00011110001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "СОДА"
}

Test! {
    Name = problem67,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["10111101", "00011110", "100111101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "СОН"
}

Test! {
    Name = problem87,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["100101000", "100000101", "0110001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АДА"
}

Test! {
    Name = problem107,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["10111101", "100111101", "0000110"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "САН"
}

Test! {
    Name = problem127,
    Input = (
        CodeType::Numbers.get(),
        Vec::from_iter(["1010110", "11110001", "100000101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ОДА"
}

Test! {
    Name = problem227,
    Input = (
        CodeType::Custom(vec![("А", "*-"), ("Г", "--*"), ("М", "--"),
                              ("К", "-*-"), ("Ю", "**--")]).get(),
        Vec::from_iter(["--*-----***---*-*-"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МАМГЮКА"
}

Test! {
    Name = problem247,
    Input = (
        CodeType::Custom(vec![("Н", "-*"), ("К", "-*-"), ("И", "**"),
                              ("Л", "*-**"), ("М", "--")]).get(),
        Vec::from_iter(["-*-*-*--**-**-*-**"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ННКНЛКИ"
}

Test! {
    Name = problem267,
    Input = (
        CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"),
                              ("Т", "-"), ("Ж", "***-")]).get(),
        Vec::from_iter(["*--***-**--**-**-*--"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АДЛТДДТАТ"
}

Test! {
    Name = problem287,
    Input = (
        CodeType::Custom(vec![("К", "+_+"), ("Л", "_*"), ("М", "*+"), ("Н", "_++"), ("О", "*"),
                              ("П", "__+"), ("Р", "__")]).get(),
        Vec::from_iter(["*+_++_++___*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МННРЛ"
}

Test! {
    Name = problem327,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["3135420", "2102030", "1331320", "2033510"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "БИТЬ"
}

Test! {
    Name = problem348,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["20335", "21120", "31321", "51201"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ДАТА"
}

Test! {
    Name = problem368,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["112233", "135793", "203014", "412030"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ГАТЬ"
}

Test! {
    Name = problem388,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["1012", "1210", "1565", "5651"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ДЕДА"
}

Test! {
    Name = problem408,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["8102030", "8112131", "8112233", "8152535"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ЖИТЬ"
}

Test! {
    Name = problem428,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["3102030", "3102033", "3112030", "3112233"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВИТЬ"
}

Test! {
    Name = problem448,
    Input = (
        CodeType::English.get(),
        Vec::from_iter(["2016", "2345", "4523", "6120"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "FAT"
}

Test! {
    Name = problem468,
    Input = (
        CodeType::English.get(),
        Vec::from_iter(["1234", "2013", "3120", "4321"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "CAT"
}

Test! {
    Name = problem488,
    Input = (
        CodeType::English.get(),
        Vec::from_iter(["18205", "20158", "20518", "81205"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "HATE"
}

Test! {
    Name = problem508,
    Input = (
        CodeType::English.get(),
        Vec::from_iter(["17205", "20127", "20217", "71205"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "GATE"
}

Test! {
    Name = problem528,
    Input = (
        CodeType::English.get(),
        Vec::from_iter(["121", "245", "913", "935"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ICE"
}

Test! {
    Name = problem548,
    Input = (
        CodeType::Custom(vec![("Р", "C?"), ("Ы", "??C"), ("Б", "??"),
                              ("К", "?C"), ("А", "?C?")]).get(),
        Vec::from_iter(["????C?C"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "БЫК"
}

Test! {
    Name = problem568,
    Input = (
        CodeType::Custom(vec![("М", "C?"), ("Ы", "?CC"), ("Ш", "??"),
                              ("К", "?C"), ("А", "?C?")]).get(),
        Vec::from_iter(["C??C??C"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МАК"
}

Test! {
    Name = problem588,
    Input = (
        CodeType::Custom(vec![("Л", "?C"), ("Е", "???"), ("Н", "CC"),
                              ("К", "C?"), ("А", "CC?")]).get(),
        Vec::from_iter(["?CCC?C?"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ЛАК"
}

Test! {
    Name = problem608,
    Input = (
        CodeType::Custom(vec![("М", "?C"), ("И", "???"), ("Ш", "CC"),
                              ("К", "C?"), ("А", "CC?")]).get(),
        Vec::from_iter(["?CCC?CC"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МАШ"
}

Test! {
    Name = problem628,
    Input = (
        CodeType::Custom(vec![("Б", "110"), ("И", "01"), ("С", "100"),
                              ("Е", "10"), ("Р", "11")]).get(),
        Vec::from_iter(["11010001100"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "БСИС"
}

Test! {
    Name = problem648,
    Input = (
        CodeType::Custom(vec![("М", "01"), ("Е", "100"), ("Т", "110"), 
                              ("Л", "101"), ("А", "10")]).get(),
        Vec::from_iter(["1101000110"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ТЕМА"
}

Test! {
    Name = problem668,
    Input = (
        CodeType::Custom(vec![("А", "10"), ("Б", "110"), ("В", "12"), ("Г", "102"), 
                              ("Д", "0"), ("Е", "22"), ("Ж", "122")]).get(),
        Vec::from_iter(["101212210102"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АВЖАГ"
}

Test! {
    Name = problem708,
    Input = (
        CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Ж", "*-**"),
                              ("Л", "-"), ("Т", "***-")]).get(),
        Vec::from_iter(["*--***-**--**-*--"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АДЖЛДЛАЛ"
}

Test! {
    Name = problem751,
    Input = (
        CodeType::Custom(vec![("К", "!!?"), ("И", "!!"), ("С", "?!"),
                              ("Л", "???"), ("О", "?!")]).get(),
        Vec::from_iter(["!!??!???"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "КОЛ"
}

Test! {
    Name = problem771,
    Input = (
        CodeType::Custom(vec![("Р", "!!?"), ("Е", "!!"), ("Д", "!?"),
                              ("И", "???"), ("С", "?!")]).get(),
        Vec::from_iter(["?!!!!?"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "СЕД"
}

Test! {
    Name = problem803,
    Input = (
        CodeType::Custom(vec![("Ш", "01"), ("К", "11"), ("О", "100"),
                              ("Л", "101"), ("А", "10")]).get(),
        Vec::from_iter(["1011011"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ЛАК"
}

Test! {
    Name = problem823,
    Input = (
        CodeType::Custom(vec![("С", "110"), ("А", "01"), ("Д", "100"), 
                              ("И", "10"), ("К", "11")]).get(),
        Vec::from_iter(["1011110"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ИКС"
}

Test! {
    Name = problem845,
    Input = (
        CodeType::Custom(vec![("П", "@@@&"), ("Р", "@&&"), ("И", "&@"),
                              ("В", "&&@"), ("Е", "&&&@"), ("Т", "@&@")]).get(),
        Vec::from_iter(["&&@&&&@@&@&&&@@&&"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВЕТЕР"
}

Test! {
    Name = problem865,
    Input = (
        CodeType::Custom(vec![("В", "@@@"), ("О", "@&"), ("Л", "&@@"),
                              ("Г", "&@&"), ("А", "&&&")]).get(),
        Vec::from_iter(["&@&@&&@@@&@@@&&&"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ГОЛОВА"
}

Test! {
    Name = problem886,
    Input = (
        CodeType::Custom(vec![("С", "110"), ("М", "10"), ("А", "00"),
                              ("О", "001"), ("Р", "101"), ("К", "010")]).get(),
        Vec::from_iter(["10001101110"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МОРС"
}

Test! {
    Name = problem906,
    Input = (
        CodeType::Custom(vec![("С", "100"), ("М", "01"), ("А", "00"), 
                              ("О", "001"), ("Р", "101"), ("К", "010")]).get(),
        Vec::from_iter(["101001010"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "РОК"
}

Test! {
    Name = problem926,
    Input = (
        CodeType::Custom(vec![("Р", "CF"), ("Ы", "FFC"), ("В", "FF"), 
                              ("О", "FC"), ("С", "FCF")]).get(),
        Vec::from_iter(["FFFCCFFCF"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВОРС"
}

Test! {
    Name = problem946,
    Input = (
        CodeType::Custom(vec![("К", "CF"), ("О", "FFC"), ("В", "FF"),
                              ("Е", "FC"), ("Р", "FCF")]).get(),
        Vec::from_iter(["FFFCCFFFC"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВЕКО"
}

Test! {
    Name = problem1018,
    Input = (
        CodeType::Custom(vec![("П", "!!?"), ("И", "!!"), ("Р", "!?"),
                              ("А", "???"), ("Т", "?!")]).get(),
        Vec::from_iter(["!?!!?!???"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "РИТА"
}

Test! {
    Name = problem1038,
    Input = (
        CodeType::Custom(vec![("С", "!!?"), ("В", "!!"), ("И", "!?"),
                              ("Т", "???"), ("Е", "?!"), ("Р", "!!!")]).get(),
        Vec::from_iter(["!!!?????!"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВИТЕ"
}

Test! {
    Name = problem1078,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["0100100101", "011011111100", "0100110001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВВОД"
}

Test! {
    Name = problem1121,
    Input = (
        CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"),
                              ("Ж", "***-"), ("Х", "****")]).get(),
        Vec::from_iter(["**-***-*--*-****-"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "УЖАТАХТ"
}

Test! {
    Name = problem1161,
    Input = (
        CodeType::Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                              ("Д", "-*"), ("Е", "+-+"), ("Ж", "**-")]).get(),
        Vec::from_iter(["*+-++-**-**"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ГБДАДА"
}

Test! {
    Name = problem4548,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["92610", "36910", "13131", "23456"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВЕЗИ"
}

Test! {
    Name = problem4585,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["11101001", "010111011", "01001010"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "РОВ"
}

Test! {
    Name = problem4644,
    Input = (
        CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"),
                              ("Г", "--*"), ("Ч", "---*")]).get(),
        Vec::from_iter(["*-**-*--*---**--*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АИНГЧАН"
}

Test! {
    Name = problem4717,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001001", "11101001", "10001010"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ДАР"
}

Test! {
    Name = problem4776,
    Input = (
        CodeType::Russian.get(),
        Vec::from_iter(["31212", "12987", "10926", "36510"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВЕДИ"
}

Test! {
    Name = problem4893,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["011111010", "01001001", "01001010"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВОР"
}

Test! {
    Name = problem4932,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["0100100101", "010111100", "10011101001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "РОД"
}

Test! {
    Name = problem5124,
    Input = (
        CodeType::Custom(vec![("М", "--"), ("Н", "-*"), ("С", "***"),
                             ("У", "**-"), ("А", "*-")]).get(),
        Vec::from_iter(["*---**--*****--*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АМУНСАН"
}

Test! {
    Name = problem5264,
    Input = (
        CodeType::Custom(vec![("К", "+-+"), ("Л", "-*"), ("М", "*+"), ("Н", "-++"),
                              ("О", "*"), ("П", "--+"), ("Р", "--")]).get(),
        Vec::from_iter(["*+-++-++---*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "МННРЛ"
}

Test! {
    Name = problem5307,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001001", "0100100101", "111011111100"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ОВОД"
}

Test! {
    Name = problem5393,
    Input = (
        CodeType::Custom(vec![("С", "***"), ("У", "**-"), ("А", "*-"),
                              ("М", "--"), ("Н", "-*")]).get(),
        Vec::from_iter(["-*--*-**-*-***--"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "НМАУАСМ"
}

Test! {
    Name = problem5552,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001010", "0100110001", "01000110001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "РУДА"
}

Test! {
    Name = problem5655,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001010", "11110001", "0100100101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ОДА"
}

Test! {
    Name = problem5775,
    Input = (
        CodeType::Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                              ("Е", "-*"), ("И", "+-+"), ("К", "**-")]).get(),
        Vec::from_iter(["*+-++-**-**"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ГБЕАЕА"
}

Test! {
    Name = problem5800,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001010", "01111110001", "10011101001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВОДА"
}

Test! {
    Name = problem5886,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["0110001", "0100110001", "10011101001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АДА"
}

Test! {
    Name = problem6260,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["11101001", "100111", "0100100101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ДО"
}

Test! {
    Name = problem6421,
    Input = (
        CodeType::Numbers2.get(),
        Vec::from_iter(["01001001", "100011111010", "10011101001"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ДВОР"
}

Test! {
    Name = problem12851,
    Input = (
        CodeType::Custom(vec![("А", "01"), ("В", "10"), ("К", "000"),
                              ("О", "111"), ("Р", "0011"), ("Т", "1101")]).get(),
        Vec::from_iter(["101110011111110101"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ВОРОТА"
}

Test! {
    Name = problem18170,
    Input = (
        CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"),
                              ("Г", "--*"), ("Ч", "---*")]).get(),
        Vec::from_iter(["*-**-*--*---**--*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "АИНГЧАН"
}

Test! {
    Name = problem18185,
    Input = (
        CodeType::Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"),
                              ("Г", "--*"), ("Ч", "---*")]).get(),
        Vec::from_iter(["-**-**--**----*"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "НАИГАЧ"
}

Test! {
    Name = problem18423,
    Input = (
        CodeType::Custom(vec![("А", "01"), ("Б", "100"), ("К", "101"),
                              ("Л", "111"), ("О", "00"), ("С", "110")]).get(),
        Vec::from_iter(["001001110110100"].map(|s| s.to_string())),
        true, false, OutputDataType::DecodedString
    ),
    Output = "ОБЛАКО"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem147,
    Input = (
        CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                              ("П", "@~+"), ("О", "+"), ("И", "~")]).get(),
        Vec::from_iter(["+~+~+@@~+"].map(|s| s.to_string())),
        false, true, OutputDataType::DecodedString
    ),
    Output = "ОЛИМП"
}

Test! {
    Name = problem167,
    Input = (
        CodeType::Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"),
                              ("И", "@~*"), ("Т", "@*"), ("О", "~*")]).get(),
        Vec::from_iter(["*@@~**~*~"].map(|s| s.to_string())),
        false, true, OutputDataType::DecodedString
    ),
    Output = "ЛИМОН"
}

Test! {
    Name = problem187,
    Input = (
        CodeType::Custom(vec![("Ж", "+#"), ("Е", "+^#"), ("С", "#"),
                              ("А", "^"), ("К", "^#"), ("Л", "#+")]).get(),
        Vec::from_iter(["#++^##^#^"].map(|s| s.to_string())),
        false, true, OutputDataType::DecodedString
    ),
    Output = "ЛЕСКА"
}

Test! {
    Name = problem207,
    Input = (
        CodeType::Custom(vec![("А", "+#"), ("Е", "#+"), ("Л", "~"),
                              ("П", "#"), ("Т", "+~#"), ("О", "~#")]).get(),
        Vec::from_iter(["#~#~#++~#"].map(|s| s.to_string())),
        false, true, OutputDataType::DecodedString
    ),
    Output = "ПОЛЕТ"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem1101,
    Input = (
        CodeType::Custom(vec![("Ж", "+#"), ("З", "+^#"), ("И", "#"),
                              ("Й", "^"), ("К", "^#"), ("Л", "#+")]).get(),
        Vec::from_iter(["#++^##^#^"].map(|s| s.to_string())),
        false, true, OutputDataType::Length
    ),
    Output = "5"
}

Test! {
    Name = problem1240,
    Input = (
        CodeType::Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"),
                              ("И", "@~*"), ("Т", "@*"), ("О", "~*")]).get(),
        Vec::from_iter(["*@@~**~*~"].map(|s| s.to_string())),
        false, true, OutputDataType::Length
    ),
    Output = "5"
}

Test! {
    Name = problem6221,
    Input = (
        CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                              ("Н", "@~+"), ("О", "+"), ("П", "~")]).get(),
        Vec::from_iter(["+~+~+@@~+"].map(|s| s.to_string())),
        false, true, OutputDataType::Length
    ),
    Output = "5"
}

Test! {
    Name = problem18256,
    Input = (
        CodeType::Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                              ("Н", "@~+"), ("О", "+"), ("П", "~")]).get(),
        Vec::from_iter(["+~+~@~+"].map(|s| s.to_string())),
        false, true, OutputDataType::Length
    ),
    Output = "4"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem1141,
    Input = (
        CodeType::Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"),
                              ("Т", "-"), ("Ж", "***-")]).get(),
        Vec::from_iter(["*--***-**--**-*--"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "8"
}

Test! {
    Name = problem4565,
    Input = (
        CodeType::Custom(vec![("Е", "*"), ("Н", "-*"), ("О", "---"),
                              ("З", "--**"), ("Щ", "--*-")]).get(),
        Vec::from_iter(["-***---*"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "5"
}

Test! {
    Name = problem4689,
    Input = (
        CodeType::Custom(vec![("А", "*-"), ("Г", "--*"), ("И", "**"),
                              ("П", "*--*"), ("М", "--")]).get(),
        Vec::from_iter(["*-*--*--**-**--*"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "6"
}

Test! {
    Name = problem4742,
    Input = (
        CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"),
                              ("Ж", "***-"), ("Х", "****")]).get(),
        Vec::from_iter(["***-**-***-*-**-"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "5"
}

Test! {
    Name = problem5311,
    Input = (
        CodeType::Custom(vec![("А", "~"), ("Б", "o++"), ("В", "oo+"), ("Г", "~+"),
                              ("Д", "o~"), ("Е", "+o+"), ("Ё", "~~o")]).get(),
        Vec::from_iter(["~+o++o~~o~~"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "6"
}

Test! {
    Name = problem6197,
    Input = (
        CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"),
                              ("Ж", "***-"), ("Х", "****")]).get(),
        Vec::from_iter(["-*-*-**--*--"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "7"
}

Test! {
    Name = problem6354,
    Input = (
        CodeType::Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"),
                              ("Ж", "***-"), ("Х", "****")]).get(),
        Vec::from_iter(["**-*-***-*--**-"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "6"
}

Test! {
    Name = problem18211,
    Input = (
        CodeType::Custom(vec![("Е", "0"), ("Н", "10"), ("О", "111"),
                              ("З", "1100"), ("Щ", "1101")]).get(),
        Vec::from_iter(["11110010011000"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "7"
}

Test! {
    Name = problem18226,
    Input = (
        CodeType::Custom(vec![("А", "01"), ("Г", "110"), ("И", "00"),
                              ("П", "0110"), ("М", "11")]).get(),
        Vec::from_iter(["0101101100100110"].map(|s| s.to_string())),
        true, false, OutputDataType::Length
    ),
    Output = "6"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem4835,
    Input = (
        CodeType::Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                              ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]).get(),
        Vec::from_iter(["...o..o.oo...o..oooo.o.oo"].map(|s| s.to_string())),
        true, false, OutputDataType::RepeatingChars
    ),
    Output = "ДЕ"
}

Test! {
    Name = problem5755,
    Input = (
        CodeType::Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                              ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]).get(),
        Vec::from_iter([".o..o.o.oo.o..o..o....o.."].map(|s| s.to_string())),
        true, false, OutputDataType::RepeatingChars
    ),
    Output = "АБ"
}
