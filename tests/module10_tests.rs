extern crate rusty_oge;

use rusty_oge::num;
use rusty_oge::module10::*;

#[macro_use]
mod test_macros;

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem16018,
    Input = (vec![num!("1100110", 2)]),
    Spec = ProblemSpec::Convert(10),
    Output = "102"
}

Test! {
    Name = problem18040,
    Input = (vec![num!("1110110", 2)]),
    Spec = ProblemSpec::Convert(10),
    Output = "118"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem10323,
    Input = (vec![num!("23", 16), num!("32", 8), num!("11110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "35"
}

Test! {
    Name = problem10324,
    Input = (vec![num!("38", 16), num!("75", 8), num!("110100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "61"
}

Test! {
    Name = problem10325,
    Input = (vec![num!("14", 16), num!("26", 8), num!("11000", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "24"
}

Test! {
    Name = problem10326,
    Input = (vec![num!("24", 16), num!("50", 8), num!("101100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "44"
}

Test! {
    Name = problem10327,
    Input = (vec![num!("50", 16), num!("106", 8), num!("1001010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "80"
}

Test! {
    Name = problem10328,
    Input = (vec![num!("50", 16), num!("106", 8), num!("1001010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "70"
}

Test! {
    Name = problem10329,
    Input = (vec![num!("41", 16), num!("77", 8), num!("1000010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "63"
}

Test! {
    Name = problem10330,
    Input = (vec![num!("32", 16), num!("60", 8), num!("110110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "48"
}

Test! {
    Name = problem10331,
    Input = (vec![num!("20", 16), num!("60", 8), num!("11100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "28"
}

Test! {
    Name = problem10333,
    Input = (vec![num!("14", 16), num!("17", 8), num!("10011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "15"
}

Test! {
    Name = problem10334,
    Input = (vec![num!("47", 16), num!("120", 8), num!("1001011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "71"
}

Test! {
    Name = problem10335,
    Input = (vec![num!("60", 16), num!("134", 8), num!("1100001", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "92"
}

Test! {
    Name = problem10336,
    Input = (vec![num!("35", 16), num!("71", 8), num!("110111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "53"
}

Test! {
    Name = problem10337,
    Input = (vec![num!("59", 16), num!("126", 8), num!("1011100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "92"
}

Test! {
    Name = problem10382,
    Input = (vec![num!("41", 16), num!("107", 8), num!("1000011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "71"
}

Test! {
    Name = problem11028,
    Input = (vec![num!("26", 16), num!("26", 8), num!("11101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "38"
}

Test! {
    Name = problem11029,
    Input = (vec![num!("28", 16), num!("47", 8), num!("101010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "39"
}

Test! {
    Name = problem11030,
    Input = (vec![num!("28", 16), num!("47", 8), num!("101010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "42"
}

Test! {
    Name = problem11031,
    Input = (vec![num!("81", 16), num!("172", 8), num!("1110011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "129"
}

Test! {
    Name = problem11032,
    Input = (vec![num!("49", 16), num!("102", 8), num!("1000111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "73"
}

Test! {
    Name = problem11033,
    Input = (vec![num!("55", 16), num!("124", 8), num!("1010101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "84"
}

Test! {
    Name = problem11034,
    Input = (vec![num!("46", 16), num!("106", 8), num!("1000101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "69"
}

Test! {
    Name = problem11035,
    Input = (vec![num!("67", 16), num!("150", 8), num!("1101000", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "103"
}

Test! {
    Name = problem11036,
    Input = (vec![num!("81", 16), num!("203", 8), num!("1111111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "127"
}

Test! {
    Name = problem11037,
    Input = (vec![num!("33", 16), num!("64", 8), num!("110100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "51"
}

Test! {
    Name = problem12859,
    Input = (vec![num!("55", 16), num!("222", 8), num!("1111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "146"
}

Test! {
    Name = problem18178,
    Input = (vec![num!("36", 16), num!("63", 8), num!("111100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "60"
}

Test! {
    Name = problem18193,
    Input = (vec![num!("47", 16), num!("73", 8), num!("101110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "71"
}

Test! {
    Name = problem18219,
    Input = (vec![num!("36", 16), num!("65", 8), num!("111010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "53"
}

Test! {
    Name = problem18234,
    Input = (vec![num!("39", 16), num!("75", 8), num!("111011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "57"
}

Test! {
    Name = problem18249,
    Input = (vec![num!("55", 10), num!("83", 10), num!("91", 10)]),
    Spec = ProblemSpec::FindDigitsSum(8, NumberToFind::Min),
    Output = "6"
}

Test! {
    Name = problem18279,
    Input = (vec![num!("59", 10), num!("71", 10), num!("81", 10)]),
    Spec = ProblemSpec::FindOnesCount(NumberToFind::Min),
    Output = "3"
}

Test! {
    Name = problem18294,
    Input = (vec![num!("100", 10), num!("90", 10), num!("80", 10)]),
    Spec = ProblemSpec::FindOnesCount(NumberToFind::Min),
    Output = "2"
}
