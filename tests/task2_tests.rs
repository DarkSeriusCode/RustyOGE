use std::collections::HashMap;
use rusty_oge::utils::SolveError;

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

// ----------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------

/// Пытается решить задание, если не получается - паникуем
fn try_solve<S>(solver: S, codes: CodeType, input: Vec<&str>, true_answer: &str)
where
    S: FnOnce(HashMap<String, String>, Vec<String>) -> Result<String, SolveError>,
{
    let codes = codes.get();
    let input = Vec::from_iter(input.iter().map(|item| item.to_string()));

    match solver(codes, input) {
        Ok(answer) => assert_eq!(answer, true_answer),
        Err(err_msg) => panic!("{err_msg}"),
    }
}

// --------------------------------------------------------------------------------------

pub mod type1 {
    use rusty_oge::task2::solvers::solve_type1;
    use super::CodeType::{self, *};

    fn try_solve(code_type: CodeType, input: Vec<&str>, true_answer: &str) {
        super::try_solve(solve_type1, code_type, input, true_answer);
    }

    #[test]
    fn problem7() {
        try_solve(Numbers, vec!["10111101", "1010110", "10111000"], "НОС");
    }

    #[test]
    fn problem27() {
        try_solve(Numbers, vec!["100101000", "101111100", "100111101"], "КОД"); 
    }

    #[test]
    fn problem47() {
        try_solve(Numbers, vec!["1010110", "100000101", "00011110001"], "СОДА");
    }

    #[test]
    fn problem67() {
        try_solve(Numbers, vec!["10111101", "00011110", "100111101"], "СОН");
    }

    #[test]
    fn problem87() {
        try_solve(Numbers, vec!["100101000", "100000101", "0110001"], "АДА");
    }

    #[test]
    fn problem107() {
        try_solve(Numbers, vec!["10111101", "100111101", "0000110"], "САН");
    }

    #[test]
    fn problem127() {
        try_solve(Numbers, vec!["1010110", "11110001", "100000101"], "ОДА");
    }

    #[test]
    fn problem227() {
        let codes = Custom(vec![("А", "*-"), ("Г", "--*"), ("М", "--"),
                                ("К", "-*-"), ("Ю", "**--")]);
        try_solve(codes, vec!["--*-----***---*-*-"], "МАМГЮКА");
    }

    #[test]
    fn problem247() {
        let codes = Custom(vec![("Н", "-*"), ("К", "-*-"), ("И", "**"),
                                ("Л", "*-**"), ("М", "--")]);
        try_solve(codes, vec!["-*-*-*--**-**-*-**"], "ННКНЛКИ");
    }

    #[test]
    fn problem267() {
        let codes = Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"),
                                ("Т", "-"), ("Ж", "***-")]);
        try_solve(codes, vec!["*--***-**--**-**-*--"], "АДЛТДДТАТ");
    }

    #[test]
    fn problem287() {
        let codes = Custom(vec![("К", "+_+"), ("Л", "_*"), ("М", "*+"), 
                                ("Н", "_++"), ("О", "*"), ("П", "__+"),
                                ("Р", "__")]);
        try_solve(codes, vec!["*+_++_++___*"], "МННРЛ");
    }

    #[test]
    fn problem327() {
        try_solve(Russian, vec!["3135420", "2102030", "1331320", "2033510"], "БИТЬ");
    }

    #[test]
    fn problem348() {
        try_solve(Russian, vec!["20335", "21120", "31321", "51201"], "ДАТА");
    }

    #[test]
    fn problem368() {
        try_solve(Russian, vec!["112233", "135793", "203014", "412030"], "ГАТЬ");
    }

    #[test]
    fn problem388() {
        try_solve(Russian, vec!["1012", "1210", "1565", "5651"], "ДЕДА");
    }

    #[test]
    fn problem408() {
        try_solve(Russian, vec!["8102030", "8112131", "8112233", "8152535"], "ЖИТЬ");
    }

    #[test]
    fn problem428() {
        try_solve(Russian, vec!["3102030", "3102033", "3112030", "3112233"], "ВИТЬ");
    }

    #[test]
    fn problem448() {
        try_solve(English, vec!["2016", "2345", "4523", "6120"], "FAT");
    }

    #[test]
    fn problem468() {
        try_solve(English, vec!["1234", "2013", "3120", "4321"], "CAT");
    }

    #[test]
    fn problem488() {
        try_solve(English, vec!["18205", "20158", "20518", "81205"], "HATE");
    }

    #[test]
    fn problem508() {
        try_solve(English, vec!["17205", "20127", "20217", "71205"], "GATE");
    }

    #[test]
    fn problem528() {
        try_solve(English, vec!["121", "245", "913", "935"], "ICE");
    }

    #[test]
    fn problem548() {
        let codes = Custom(vec![("Р", "C?"), ("Ы", "??C"), ("Б", "??"),
                                ("К", "?C"), ("А", "?C?")]);
        try_solve(codes, vec!["????C?C"], "БЫК");
    }

    #[test]
    fn problem568() {
        let codes = Custom(vec![("М", "C?"), ("Ы", "?CC"), ("Ш", "??"),
                                ("К", "?C"), ("А", "?C?")]);
        try_solve(codes, vec!["C??C??C"], "МАК");
    }

    #[test]
    fn problem588() {
        let codes = Custom(vec![("Л", "?C"), ("Е", "???"), ("Н", "CC"),
                                ("К", "C?"), ("А", "CC?")]);
        try_solve(codes, vec!["?CCC?C?"], "ЛАК");
    }

    #[test]
    fn problem608() {
        let codes = Custom(vec![("М", "?C"), ("И", "???"), ("Ш", "CC"),
                                ("К", "C?"), ("А", "CC?")]);
        try_solve(codes, vec!["?CCC?CC"], "МАШ");
    }

    #[test]
    fn problem628() {
        let codes = Custom(vec![("Б", "110"), ("И", "01"), ("С", "100"),
                                ("Е", "10"), ("Р", "11")]);
        try_solve(codes, vec!["11010001100"], "БСИС");
    }

    #[test]
    fn problem648() {
        let codes = Custom(vec![("М", "01"), ("Е", "100"), ("Т", "110"), 
                                ("Л", "101"), ("А", "10")]);
        try_solve(codes, vec!["1101000110"], "ТЕМА");
    }

    #[test]
    fn problem668() {
        let codes = Custom(vec![("А", "10"), ("Б", "110"), ("В", "12"), 
                                ("Г", "102"), ("Д", "0"), ("Е", "22"), ("Ж", "122")]);
        try_solve(codes, vec!["101212210102"], "АВЖАГ")
    }

    #[test]
    fn problem708() {
        let codes = Custom(vec![("А", "*-"), ("Д", "-**"), ("Ж", "*-**"),
                                ("Л", "-"), ("Т", "***-")]);
        try_solve(codes, vec!["*--***-**--**-*--"], "АДЖЛДЛАЛ");
    }

    #[test]
    fn problem751() {
        let codes = Custom(vec![("К", "!!?"), ("И", "!!"), ("С", "?!"),
                                ("Л", "???"), ("О", "?!")]);
        try_solve(codes, vec!["!!??!???"], "КОЛ");
    }

    #[test]
    fn problem771() {
        let codes = Custom(vec![("Р", "!!?"), ("Е", "!!"), ("Д", "!?"),
                                ("И", "???"), ("С", "?!")]);
        try_solve(codes, vec!["?!!!!?"], "СЕД");
    }

    #[test]
    fn problem803() {
        let codes = Custom(vec![("Ш", "01"), ("К", "11"), ("О", "100"),
                                ("Л", "101"), ("А", "10")]);
        try_solve(codes, vec!["1011011"], "ЛАК");
    }

    #[test]
    fn problem823() {
        let codes = Custom(vec![("С", "110"), ("А", "01"), ("Д", "100"), 
                                ("И", "10"), ("К", "11")]);
        try_solve(codes, vec!["1011110"], "ИКС");
    }

    #[test]
    fn problem845() {
        let codes = Custom(vec![("П", "@@@&"), ("Р", "@&&"), ("И", "&@"),
                                ("В", "&&@"), ("Е", "&&&@"), ("Т", "@&@")]);
        try_solve(codes, vec!["&&@&&&@@&@&&&@@&&"], "ВЕТЕР");
    }

    #[test]
    fn problem865() {
        let codes = Custom(vec![("В", "@@@"), ("О", "@&"), ("Л", "&@@"),
                                ("Г", "&@&"), ("А", "&&&")]);
        try_solve(codes, vec!["&@&@&&@@@&@@@&&&"], "ГОЛОВА");
    }

    #[test]
    fn problem886() {
        let codes = Custom(vec![("С", "110"), ("М", "10"), ("А", "00"),
                                ("О", "001"), ("Р", "101"), ("К", "010")]);
        try_solve(codes, vec!["10001101110"], "МОРС");
    }

    #[test]
    fn problem906() {
        let codes = Custom(vec![("С", "100"), ("М", "01"), ("А", "00"), 
                                ("О", "001"), ("Р", "101"), ("К", "010")]);
        try_solve(codes, vec!["101001010"], "РОК");
    }

    #[test]
    fn problem926() {
        let codes = Custom(vec![("Р", "CF"), ("Ы", "FFC"), ("В", "FF"), 
                                ("О", "FC"), ("С", "FCF")]);
        try_solve(codes, vec!["FFFCCFFCF"], "ВОРС");
    }

    #[test]
    fn problem946() {
        let codes = Custom(vec![("К", "CF"), ("О", "FFC"), ("В", "FF"),
                                ("Е", "FC"), ("Р", "FCF")]);
        try_solve(codes, vec!["FFFCCFFFC"], "ВЕКО");
    }

    #[test]
    fn problem1018() {
        let codes = Custom(vec![("П", "!!?"), ("И", "!!"), ("Р", "!?"),
                                ("А", "???"), ("Т", "?!")]);
        try_solve(codes, vec!["!?!!?!???"], "РИТА");
    }

    #[test]
    fn problem1038() {
        let codes = Custom(vec![("С", "!!?"), ("В", "!!"), ("И", "!?"),
                                ("Т", "???"), ("Е", "?!"), ("Р", "!!!")]);
        try_solve(codes, vec!["!!!?????!"], "ВИТЕ");
    }

    #[test]
    fn problem1078() {
        try_solve(Numbers2, vec!["0100100101", "011011111100", "0100110001"], "ВВОД");
    }

    #[test]
    fn problem1121() {
        let codes = Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                                ("Х", "****")]);
        try_solve(codes, vec!["**-***-*--*-****-"], "УЖАТАХТ");
    }

    #[test]
    fn problem1161() {
        let codes = Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                                ("Д", "-*"), ("Е", "+-+"), ("Ж", "**-")]);
        try_solve(codes, vec!["*+-++-**-**"], "ГБДАДА");
    }

    #[test]
    fn problem4548() {
        try_solve(Russian, vec!["92610", "36910", "13131", "23456"], "ВЕЗИ");
    }

    #[test]
    fn problem4585() {
        try_solve(Numbers2, vec!["11101001", "010111011", "01001010"], "РОВ");
    }

    #[test]
    fn problem4644() {
        let codes = Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                                ("Ч", "---*")]);
        try_solve(codes, vec!["*-**-*--*---**--*"], "АИНГЧАН");
    }

    #[test]
    fn problem4717() {
        try_solve(Numbers2, vec!["01001001", "11101001", "10001010"], "ДАР");
    }

    #[test]
    fn problem4776() {
        try_solve(Russian, vec!["31212", "12987", "10926", "36510"], "ВЕДИ");
    }

    #[test]
    fn problem4893() {
        try_solve(Numbers2, vec!["011111010", "01001001", "01001010"], "ВОР");
    }

    #[test]
    fn problem4932() {
        try_solve(Numbers2, vec!["0100100101", "010111100", "10011101001"], "РОД");
    }

    #[test]
    fn problem5124() {
        let codes = Custom(vec![("М", "--"), ("Н", "-*"), ("С", "***"), ("У", "**-"),
                                ("А", "*-")]);
        try_solve(codes, vec!["*---**--*****--*"], "АМУНСАН");
    }

    #[test]
    fn problem5264() {
        let codes = Custom(vec![("К", "+-+"), ("Л", "-*"), ("М", "*+"), ("Н", "-++"),
                                ("О", "*"), ("П", "--+"), ("Р", "--")]);
        try_solve(codes, vec!["*+-++-++---*"], "МННРЛ");
    }

    #[test]
    fn problem5307() {
        try_solve(Numbers2, vec!["01001001", "0100100101", "111011111100"], "ОВОД");
    }

    #[test]
    fn problem5393() {
        let codes = Custom(vec![("С", "***"), ("У", "**-"), ("А", "*-"), ("М", "--"),
                                ("Н", "-*")]);
        try_solve(codes, vec!["-*--*-**-*-***--"], "НМАУАСМ");
    }

    #[test]
    fn problem5552() {
        try_solve(Numbers2, vec!["01001010", "0100110001", "01000110001"], "РУДА");
    }

    #[test]
    fn problem5655() {
        try_solve(Numbers2, vec!["01001010", "11110001", "0100100101"], "ОДА");
    }

    #[test]
    fn problem5775() {
        let codes = Custom(vec![("А", "*"), ("Б", "-++"), ("В", "--+"), ("Г", "*+"),
                                ("Е", "-*"), ("И", "+-+"), ("К", "**-")]);
        try_solve(codes, vec!["*+-++-**-**"], "ГБЕАЕА");
    }

    #[test]
    fn problem5800() {
        try_solve(Numbers2, vec!["01001010", "01111110001", "10011101001"], "ВОДА")
    }

    #[test]
    fn problem5886() {
        try_solve(Numbers2, vec!["0110001", "0100110001", "10011101001"], "АДА");
    }

    #[test]
    fn problem6260() {
        try_solve(Numbers2, vec!["11101001", "100111", "0100100101"], "ДО");
    }

    #[test]
    fn problem6421() {
        try_solve(Numbers2, vec!["01001001", "100011111010", "10011101001"], "ДВОР");
    }

    #[test]
    fn problem12851() {
        let codes = Custom(vec![("А", "01"), ("В", "10"), ("К", "000"), ("О", "111"),
                                ("Р", "0011"), ("Т", "1101")]);
        try_solve(codes, vec!["101110011111110101"], "ВОРОТА");
    }

    #[test]
    fn problem18170() {
        let codes = Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                                ("Ч", "---*")]);
        try_solve(codes, vec!["*-**-*--*---**--*"], "АИНГЧАН");
    }

    #[test]
    fn problem18185() {
        let codes = Custom(vec![("И", "**"), ("А", "*-"), ("Н", "-*"), ("Г", "--*"),
                                ("Ч", "---*")]);
        try_solve(codes, vec!["-**-**--**----*"], "НАИГАЧ");
    }

    #[test]
    fn problem18423() {
        let codes = Custom(vec![("А", "01"), ("Б", "100"), ("К", "101"), ("Л", "111"),
                                ("О", "00"), ("С", "110")]);
        try_solve(codes, vec!["001001110110100"], "ОБЛАКО");
    }
}

pub mod type2 {
    use rusty_oge::task2::solvers::solve_type2;
    use super::CodeType::{self, Custom};

    fn try_solve(codes: CodeType, input: &str, true_answer: &str) {
        super::try_solve(solve_type2, codes, vec![input], true_answer);
    }

    #[test]
    fn problem147() {
        let codes = Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                                   ("П", "@~+"), ("О", "+"), ("И", "~")]);
        try_solve(codes, "+~+~+@@~+", "ОЛИМП");
    }

    #[test]
    fn problem167() {
        let codes = Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"),
                                   ("И", "@~*"), ("Т", "@*"), ("О", "~*")]);
        try_solve(codes, "*@@~**~*~", "ЛИМОН");
    }

    #[test]
    fn problem187() {
        let codes = Custom(vec![("Ж", "+#"), ("Е", "+^#"), ("С", "#"),
                                    ("А", "^"), ("К", "^#"), ("Л", "#+")]);
        try_solve(codes, "#++^##^#^", "ЛЕСКА");
    }

    #[test]
    fn problem207() {
        let codes = Custom(vec![("А", "+#"), ("Е", "#+"), ("Л", "~"),
                                    ("П", "#"), ("Т", "+~#"), ("О", "~#")]);
        try_solve(codes, "#~#~#++~#", "ПОЛЕТ");
    }
}

pub mod type3 {
    use rusty_oge::task2::solvers::solve_type3;
    use super::CodeType::{self, Custom};

    fn try_solve(codes: CodeType, input: &str, true_answer: &str) {
        super::try_solve(solve_type3, codes, vec![input], true_answer);
    }

    #[test]
    fn problem1101() {
        let codes = Custom(vec![("Ж", "+#"), ("З", "+^#"), ("И", "#"), ("Й", "^"),
                                ("К", "^#"), ("Л", "#+")]);
        try_solve(codes, "#++^##^#^", "5");
    }

    #[test]
    fn problem1240() {
        let codes = Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"), ("И", "@~*"),
                                ("Т", "@*"), ("О", "~*")]);
        try_solve(codes, "*@@~**~*~", "5");
    }

    #[test]
    fn problem6221() {
        let codes = Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"), ("Н", "@~+"),
                                ("О", "+"), ("П", "~")]);
        try_solve(codes, "+~+~+@@~+", "5");
    }

    #[test]
    fn problem18256() {
        let codes = Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"), ("Н", "@~+"),
                                ("О", "+"), ("П", "~")]);
        try_solve(codes, "+~+~@~+", "4");
    }
}

pub mod type4 {
    use rusty_oge::task2::solvers::solve_type4;
    use super::CodeType::{self, Custom};

    fn try_solve(codes: CodeType, input: &str, true_answer: &str) {
        super::try_solve(solve_type4, codes, vec![input], true_answer);
    }

    #[test]
    fn problem1141() {
        let codes = Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"), ("Т", "-"),
                                ("Ж", "***-")]);
        try_solve(codes, "*--***-**--**-*--", "8");
    }

    #[test]
    fn problem4565() {
        let codes = Custom(vec![("Е", "*"), ("Н", "-*"), ("О", "---"), ("З", "--**"),
                                ("Щ", "--*-")]);
        try_solve(codes, "-***---*", "5");
    }

    #[test]
    fn problem4689() {
        let codes = Custom(vec![("А", "*-"), ("Г", "--*"), ("И", "**"), ("П", "*--*"),
                                ("М", "--")]);
        try_solve(codes, "*-*--*--**-**--*", "6");
    }

    #[test]
    fn problem4742() {
        let codes = Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                                ("Х", "****")]);
        try_solve(codes, "***-**-***-*-**-", "5");
    }

    #[test]
    fn problem5311() {
        let codes = Custom(vec![("А", "~"), ("Б", "o++"), ("В", "oo+"), ("Г", "~+"),
                                ("Д", "o~"), ("Е", "+o+"), ("Ё", "~~o")]);
        try_solve(codes, "~+o++o~~o~~", "6");
    }

    #[test]
    fn problem6197() {
        let codes = Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                                ("Х", "****")]);
        try_solve(codes, "-*-*-**--*--", "7");
    }

    #[test]
    fn problem6354() {
        let codes = Custom(vec![("Т", "-"), ("А", "*-"), ("У", "**-"), ("Ж", "***-"),
                                ("Х", "****")]);
        try_solve(codes, "**-*-***-*--**-", "6");
    }

    #[test]
    fn problem18211() {
        let codes = Custom(vec![("Е", "0"), ("Н", "10"), ("О", "111"), ("З", "1100"),
                                ("Щ", "1101")]);
        try_solve(codes, "11110010011000", "7");
    }

    #[test]
    fn problem18226() {
        let codes = Custom(vec![("А", "01"), ("Г", "110"), ("И", "00"), ("П", "0110"),
                                ("М", "11")]);
        try_solve(codes, "0101101100100110", "6");
    }
}

pub mod type5 {
    use rusty_oge::task2::solvers::solve_type5;
    use super::CodeType::{self, Custom};

    fn try_solve(codes: CodeType, input: &str, true_answer: &str) {
        super::try_solve(solve_type5, codes, vec![input], true_answer);
    }

    #[test]
    fn problem4835() {
        let codes = Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                                ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]);
        try_solve(codes, "...o..o.oo...o..oooo.o.oo", "ДЕ");
    }

    #[test]
    fn problem5755() {
        let codes = Custom(vec![("А", "..o.."), ("Б", ".o..o"), ("В", ".oo.o"),
                                ("Г", ".oooo"), ("Д", "...o."), ("Е", ".o.oo")]);
        try_solve(codes, ".o..o.o.oo.o..o..o....o..", "БА");
    }
}
