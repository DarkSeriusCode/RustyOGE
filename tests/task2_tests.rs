/// Обозначения для групп тестов
/// type1: Задания, схожие с https://inf-oge.sdamgia.ru/problem?id=7 и 
///        https://inf-oge.sdamgia.ru/problem?id=327
///
/// type2: Задания, схожие с https://inf-oge.sdamgia.ru/problem?id=147

use std::collections::HashMap;

// ----------------------------------------------------------------------------

const NUM_CODES: [(&str, &str); 6] = [("А", "01"), ("Д", "100"), ("К", "101"),
                                      ("Н", "10"), ("О", "111"), ("С", "000")];

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
    Russian,
    English,
    Custom(Vec<(&'static str, &'static str)>),
}

impl CodeType {
    fn get(&self) -> HashMap<String, String> {
        get_codes(match self {
            Self::Numbers => Vec::from(NUM_CODES),
            Self::Russian => Vec::from(RUS_CODES),
            Self::English => Vec::from(ENG_CODES),
            Self::Custom(v) => v.clone(),
        })
    }
}

// ----------------------------------------------------------------------------

pub mod type1 {
    use rusty_oge::task2::solvers;
    use super::CodeType::{self, Numbers, Russian, English, Custom};

    /// Пытается решить задание, если не получается - паникуем
    fn try_solve(code_type: CodeType, input: Vec<&str>, true_answer: &str) {
        let codes = code_type.get();
        let input = Vec::from_iter(input.iter().map(|item| item.to_string()));
        let answer = solvers::solve_type1(codes, input);

        match answer {
            Ok(a) => assert_eq!(true_answer, a),
            Err(msg) => panic!("{msg}"),
        }
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=7
    fn problem7() {
        try_solve(Numbers, vec!["10111101", "1010110", "10111000"], "НОС");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=27
    fn problem27() {
        try_solve(Numbers, vec!["100101000", "101111100", "100111101"], "КОД"); 
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=47
    fn problem47() {
        try_solve(Numbers, vec!["1010110", "100000101", "00011110001"], "СОДА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=67
    fn problem67() {
        try_solve(Numbers, vec!["10111101", "00011110", "100111101"], "СОН");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=87
    fn problem87() {
        try_solve(Numbers, vec!["100101000", "100000101", "0110001"], "АДА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=107
    fn problem107() {
        try_solve(Numbers, vec!["10111101", "100111101", "0000110"], "САН");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=127
    fn problem127() {
        try_solve(Numbers, vec!["1010110", "11110001", "100000101"], "ОДА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=227
    fn problem227() {
        let codes = Custom(vec![("А", "*-"), ("Г", "--*"), ("М", "--"),
                                ("К", "-*-"), ("Ю", "**--")]);
        try_solve(codes, vec!["--*-----***---*-*-"], "МАМГЮКА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=247
    fn problem247() {
        let codes = Custom(vec![("Н", "-*"), ("К", "-*-"), ("И", "**"),
                                ("Л", "*-**"), ("М", "--")]);
        try_solve(codes, vec!["-*-*-*--**-**-*-**"], "ННКНЛКИ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=267
    fn problem267() {
        let codes = Custom(vec![("А", "*-"), ("Д", "-**"), ("Л", "*-**"),
                                ("Т", "-"), ("Ж", "***-")]);
        try_solve(codes, vec!["*--***-**--**-**-*--"], "АДЛТДДТАТ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=287
    fn problem287() {
        let codes = Custom(vec![("К", "+_+"), ("Л", "_*"), ("М", "*+"), 
                                ("Н", "_++"), ("О", "*"), ("П", "__+"),
                                ("Р", "__")]);
        try_solve(codes, vec!["*+_++_++___*"], "МННРЛ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=327
    fn problem327() {
        try_solve(Russian, vec!["3135420", "2102030", "1331320", "2033510"], "БИТЬ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=348
    fn problem348() {
        try_solve(Russian, vec!["20335", "21120", "31321", "51201"], "ДАТА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=368
    fn problem368() {
        try_solve(Russian, vec!["112233", "135793", "203014", "412030"], "ГАТЬ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=388
    fn problem388() {
        try_solve(Russian, vec!["1012", "1210", "1565", "5651"], "ДЕДА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=408
    fn problem408() {
        try_solve(Russian, vec!["8102030", "8112131", "8112233", "8152535"], "ЖИТЬ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=428
    fn problem428() {
        try_solve(Russian, vec!["3102030", "3102033", "3112030", "3112233"], "ВИТЬ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=448
    fn problem448() {
        try_solve(English, vec!["2016", "2345", "4523", "6120"], "FAT");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=468
    fn problem468() {
        try_solve(English, vec!["1234", "2013", "3120", "4321"], "CAT");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=488
    fn problem488() {
        try_solve(English, vec!["18205", "20158", "20518", "81205"], "HATE");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=508
    fn problem508() {
        try_solve(English, vec!["17205", "20127", "20217", "71205"], "GATE");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=528
    fn problem528() {
        try_solve(English, vec!["121", "245", "913", "935"], "ICE");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=548
    fn problem548() {
        let codes = Custom(vec![("Р", "C?"), ("Ы", "??C"), ("Б", "??"),
                                ("К", "?C"), ("А", "?C?")]);
        try_solve(codes, vec!["????C?C"], "БЫК");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=568
    fn problem568() {
        let codes = Custom(vec![("М", "C?"), ("Ы", "?CC"), ("Ш", "??"),
                                ("К", "?C"), ("А", "?C?")]);
        try_solve(codes, vec!["C??C??C"], "МАК");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=588
    fn problem588() {
        let codes = Custom(vec![("Л", "?C"), ("Е", "???"), ("Н", "CC"),
                                ("К", "C?"), ("А", "CC?")]);
        try_solve(codes, vec!["?CCC?C?"], "ЛАК");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=608
    fn problem608() {
        let codes = Custom(vec![("М", "?C"), ("И", "???"), ("Ш", "CC"),
                                ("К", "C?"), ("А", "CC?")]);
        try_solve(codes, vec!["?CCC?CC"], "МАШ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=628
    fn problem628() {
        let codes = Custom(vec![("Б", "110"), ("И", "01"), ("С", "100"),
                                ("Е", "10"), ("Р", "11")]);
        try_solve(codes, vec!["11010001100"], "БСИС");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=648
    fn problem648() {
        let codes = Custom(vec![("М", "01"), ("Е", "100"), ("Т", "110"), 
                                ("Л", "101"), ("А", "10")]);
        try_solve(codes, vec!["1101000110"], "ТЕМА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=668
    fn problem668() {
        let codes = Custom(vec![("А", "10"), ("Б", "110"), ("В", "12"), 
                                ("Г", "102"), ("Д", "0"), ("Е", "22"), ("Ж", "122")]);
        try_solve(codes, vec!["101212210102"], "АВЖАГ")
    }
    
    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=708
    fn problem708() {
        let codes = Custom(vec![("А", "*-"), ("Д", "-**"), ("Ж", "*-**"),
                                ("Л", "-"), ("Т", "***-")]);
        try_solve(codes, vec!["*--***-**--**-*--"], "АДЖЛДЛАЛ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=751
    fn problem751() {
        let codes = Custom(vec![("К", "!!?"), ("И", "!!"), ("С", "?!"),
                                ("Л", "???"), ("О", "?!")]);
        try_solve(codes, vec!["!!??!???"], "КОЛ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=771
    fn problem771() {
        let codes = Custom(vec![("Р", "!!?"), ("Е", "!!"), ("Д", "!?"),
                                ("И", "???"), ("С", "?!")]);
        try_solve(codes, vec!["?!!!!?"], "СЕД");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=803
    fn problem803() {
        let codes = Custom(vec![("Ш", "01"), ("К", "11"), ("О", "100"),
                                ("Л", "101"), ("А", "10")]);
        try_solve(codes, vec!["1011011"], "ЛАК");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=823
    fn problem823() {
        let codes = Custom(vec![("С", "110"), ("А", "01"), ("Д", "100"), 
                                ("И", "10"), ("К", "11")]);
        try_solve(codes, vec!["1011110"], "ИКС");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=845
    fn problem845() {
        let codes = Custom(vec![("П", "@@@&"), ("Р", "@&&"), ("И", "&@"),
                                ("В", "&&@"), ("Е", "&&&@"), ("Т", "@&@")]);
        try_solve(codes, vec!["&&@&&&@@&@&&&@@&&"], "ВЕТЕР");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=865
    fn problem865() {
        let codes = Custom(vec![("В", "@@@"), ("О", "@&"), ("Л", "&@@"),
                                ("Г", "&@&"), ("А", "&&&")]);
        try_solve(codes, vec!["&@&@&&@@@&@@@&&&"], "ГОЛОВА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=886
    fn problem886() {
        let codes = Custom(vec![("С", "110"), ("М", "10"), ("А", "00"),
                                ("О", "001"), ("Р", "101"), ("К", "010")]);
        try_solve(codes, vec!["10001101110"], "МОРС");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=906
    fn problem906() {
        let codes = Custom(vec![("С", "100"), ("М", "01"), ("А", "00"), 
                                ("О", "001"), ("Р", "101"), ("К", "010")]);
        try_solve(codes, vec!["101001010"], "РОК");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=926
    fn problem926() {
        let codes = Custom(vec![("Р", "CF"), ("Ы", "FFC"), ("В", "FF"), 
                                ("О", "FC"), ("С", "FCF")]);
        try_solve(codes, vec!["FFFCCFFCF"], "ВОРС");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=946
    fn problem946() {
        let codes = Custom(vec![("К", "CF"), ("О", "FFC"), ("В", "FF"),
                                ("Е", "FC"), ("Р", "FCF")]);
        try_solve(codes, vec!["FFFCCFFFC"], "ВЕКО");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=1018
    fn problem1018() {
        let codes = Custom(vec![("П", "!!?"), ("И", "!!"), ("Р", "!?"),
                                ("А", "???"), ("Т", "?!")]);
        try_solve(codes, vec!["!?!!?!???"], "РИТА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=1038
    fn problem1038() {
        let codes = Custom(vec![("С", "!!?"), ("В", "!!"), ("И", "!?"),
                                ("Т", "???"), ("Е", "?!"), ("Р", "!!!")]);
        try_solve(codes, vec!["!!!?????!"], "ВИТЕ");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=1078
    fn problem1078() {
        let codes = Custom(vec![("А", "01"), ("В", "011"), ("Д", "100"),
                                ("О", "111"), ("Р", "010"), ("У", "001")]);
        try_solve(codes, vec!["0100100101", "011011111100", "0100110001"], "ВВОД");
    }
}

pub mod type2 {
    use rusty_oge::task2::solvers;
    use super::CodeType::{self, Custom};

    fn try_solve(codes: CodeType, input: &str, true_answer: &str) {
        let answer = solvers::solve_type2(codes.get(), vec![input.to_string()]);

        match answer {
            Ok(a) => assert_eq!(true_answer, a),
            Err(msg) => panic!("{msg}"),
        };
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=147
    fn problem147() {
        let codes = Custom(vec![("К", "@+"), ("Л", "~+"), ("М", "+@"),
                                   ("П", "@~+"), ("О", "+"), ("И", "~")]);
        try_solve(codes, "+~+~+@@~+", "ОЛИМП");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=167
    fn problem167() {
        let codes = Custom(vec![("Н", "~"), ("М", "*"), ("Л", "*@"),
                                   ("И", "@~*"), ("Т", "@*"), ("О", "~*")]);
        try_solve(codes, "*@@~**~*~", "ЛИМОН");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=187
    fn problem187() {
        let codes = Custom(vec![("Ж", "+#"), ("Е", "+^#"), ("С", "#"),
                                    ("А", "^"), ("К", "^#"), ("Л", "#+")]);
        try_solve(codes, "#++^##^#^", "ЛЕСКА");
    }

    #[test]
    /// https://inf-oge.sdamgia.ru/problem?id=207
    fn problem207() {
        let codes = Custom(vec![("А", "+#"), ("Е", "#+"), ("Л", "~"),
                                    ("П", "#"), ("Т", "+~#"), ("О", "~#")]);
        try_solve(codes, "#~#~#++~#", "ПОЛЕТ");
    }
}

