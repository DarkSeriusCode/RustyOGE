extern crate rusty_oge;

use rusty_oge::module10::*;
use rusty_oge::utils::Validated;

#[macro_use]
mod test_macros;

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem16018,
    Input = (vec![Number::new("1100110", 2)]),
    Spec = ProblemSpec::Convert(10),
    Output = "102"
}

Test! {
    Name = problem18040,
    Input = (vec![Number::new("1110110", 2)]),
    Spec = ProblemSpec::Convert(10),
    Output = "118"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem10323,
    Input = (vec![Number::new("23", 16), Number::new("32", 8), Number::new("11110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "35"
}

Test! {
    Name = problem10324,
    Input = (vec![Number::new("38", 16), Number::new("75", 8), Number::new("110100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "61"
}

Test! {
    Name = problem10325,
    Input = (vec![Number::new("14", 16), Number::new("26", 8), Number::new("11000", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "24"
}

Test! {
    Name = problem10326,
    Input = (vec![Number::new("24", 16), Number::new("50", 8), Number::new("101100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "44"
}

Test! {
    Name = problem10327,
    Input = (vec![Number::new("50", 16), Number::new("106", 8), Number::new("1001010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "80"
}

Test! {
    Name = problem10328,
    Input = (vec![Number::new("50", 16), Number::new("106", 8), Number::new("1001010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "70"
}

Test! {
    Name = problem10329,
    Input = (vec![Number::new("41", 16), Number::new("77", 8), Number::new("1000010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "63"
}

Test! {
    Name = problem10330,
    Input = (vec![Number::new("32", 16), Number::new("60", 8), Number::new("110110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "48"
}

Test! {
    Name = problem10331,
    Input = (vec![Number::new("20", 16), Number::new("60", 8), Number::new("11100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "28"
}

Test! {
    Name = problem10333,
    Input = (vec![Number::new("14", 16), Number::new("17", 8), Number::new("10011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "15"
}

Test! {
    Name = problem10334,
    Input = (vec![Number::new("47", 16), Number::new("120", 8), Number::new("1001011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "71"
}

Test! {
    Name = problem10335,
    Input = (vec![Number::new("60", 16), Number::new("134", 8), Number::new("1100001", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "92"
}

Test! {
    Name = problem10336,
    Input = (vec![Number::new("35", 16), Number::new("71", 8), Number::new("110111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "53"
}

Test! {
    Name = problem10337,
    Input = (vec![Number::new("59", 16), Number::new("126", 8), Number::new("1011100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "92"
}

Test! {
    Name = problem10382,
    Input = (vec![Number::new("41", 16), Number::new("107", 8), Number::new("1000011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "71"
}

Test! {
    Name = problem11028,
    Input = (vec![Number::new("26", 16), Number::new("26", 8), Number::new("11101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "38"
}

Test! {
    Name = problem11029,
    Input = (vec![Number::new("28", 16), Number::new("47", 8), Number::new("101010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "39"
}

Test! {
    Name = problem11030,
    Input = (vec![Number::new("28", 16), Number::new("47", 8), Number::new("101010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "42"
}

Test! {
    Name = problem11031,
    Input = (vec![Number::new("81", 16), Number::new("172", 8), Number::new("1110011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "129"
}

Test! {
    Name = problem11032,
    Input = (vec![Number::new("49", 16), Number::new("102", 8), Number::new("1000111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "73"
}

Test! {
    Name = problem11033,
    Input = (vec![Number::new("55", 16), Number::new("124", 8), Number::new("1010101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "84"
}

Test! {
    Name = problem11034,
    Input = (vec![Number::new("46", 16), Number::new("106", 8), Number::new("1000101", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "69"
}

Test! {
    Name = problem11035,
    Input = (vec![Number::new("67", 16), Number::new("150", 8), Number::new("1101000", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "103"
}

Test! {
    Name = problem11036,
    Input = (vec![Number::new("81", 16), Number::new("203", 8), Number::new("1111111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "127"
}

Test! {
    Name = problem11037,
    Input = (vec![Number::new("33", 16), Number::new("64", 8), Number::new("110100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "51"
}

Test! {
    Name = problem12859,
    Input = (vec![Number::new("55", 16), Number::new("222", 8), Number::new("1111", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "146"
}

Test! {
    Name = problem18178,
    Input = (vec![Number::new("36", 16), Number::new("63", 8), Number::new("111100", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "60"
}

Test! {
    Name = problem18193,
    Input = (vec![Number::new("47", 16), Number::new("73", 8), Number::new("101110", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Max),
    Output = "71"
}

Test! {
    Name = problem18219,
    Input = (vec![Number::new("36", 16), Number::new("65", 8), Number::new("111010", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "53"
}

Test! {
    Name = problem18234,
    Input = (vec![Number::new("39", 16), Number::new("75", 8), Number::new("111011", 2)]),
    Spec = ProblemSpec::FindNum(NumberToFind::Min),
    Output = "57"
}

Test! {
    Name = problem18249,
    Input = (vec![Number::new("55", 10), Number::new("83", 10), Number::new("91", 10)]),
    Spec = ProblemSpec::FindDigitsSum(8, NumberToFind::Min),
    Output = "6"
}

Test! {
    Name = problem18279,
    Input = (vec![Number::new("59", 10), Number::new("71", 10), Number::new("81", 10)]),
    Spec = ProblemSpec::FindOnesCount(NumberToFind::Min),
    Output = "3"
}

Test! {
    Name = problem18294,
    Input = (vec![Number::new("100", 10), Number::new("90", 10), Number::new("80", 10)]),
    Spec = ProblemSpec::FindOnesCount(NumberToFind::Min),
    Output = "2"
}
