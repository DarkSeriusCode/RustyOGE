extern crate rusty_oge;

use rusty_oge::module3::*;
use rusty_oge::utils::NumberToFind;

#[macro_use]
mod test_macros;

Test! {
    Name = problem10638,
    Input = (NumberToFind::Max, None, "НЕ (X <= 15) И (X < 20).", true),
    Output = "19"
}

Test! {
    Name = problem10639,
    Input = (NumberToFind::Max, None, "НЕ (X <= 14) И (X <= 18).", true),
    Output = "18"
}

Test! {
    Name = problem10640,
    Input = (NumberToFind::Max, None, "НЕ (X <= 10) И НЕ (X > 16).", true),
    Output = "16"
}

Test! {
    Name = problem10641,
    Input = (NumberToFind::Max, None, "НЕ (X <= 6) И НЕ (X >= 11).", true),
    Output = "10"
}

Test! {
    Name = problem10642,
    Input = (NumberToFind::Max, None, "НЕ (X <= 3) И НЕ (X >= 7).", true),
    Output = "6"
}

Test! {
    Name = problem10643,
    Input = (NumberToFind::Max, None, "НЕ (X четное) И НЕ (X >= 7).", true),
    Output = "5"
}

Test! {
    Name = problem10644,
    Input = (NumberToFind::Max, None, "НЕ (X четное) И НЕ (X >= 11).", true),
    Output = "9"
}

Test! {
    Name = problem10645,
    Input = (NumberToFind::Max, None, "НЕ (X нечетное) И НЕ (X >= 10).", true),
    Output = "8"
}

Test! {
    Name = problem10646,
    Input = (NumberToFind::Max, None, "НЕ (X нечетное) И НЕ (X >= 6).", true),
    Output = "4"
}

Test! {
    Name = problem10868,
    Input = (NumberToFind::Max, None, "(X >= 6) И НЕ (X > 12).", true),
    Output = "12"
}

Test! {
    Name = problem10869,
    Input = (NumberToFind::Max, None, "(X > 5) И НЕ (X > 15).", true),
    Output = "15"
}


Test! {
    Name = problem10873,
    Input = (NumberToFind::Max, None, "НЕ (X < 4) И НЕ (X >= 9).", true),
    Output = "8"
}

Test! {
    Name = problem10874,
    Input = (NumberToFind::Max, None, "НЕ (X <= 13) И НЕ (X >= 19).", true),
    Output = "18"
}

Test! {
    Name = problem10875,
    Input = (NumberToFind::Max, None, "НЕ (X <= 11) И НЕ (X >= 17).", true),
    Output = "16"
}

Test! {
    Name = problem10876,
    Input = (NumberToFind::Max, None, "НЕ (X <= 11) И НЕ (X >= 17) И (X нечетное).", true),
    Output = "15"
}

Test! {
    Name = problem12852,
    Input = (NumberToFind::Max, None, "(x < 17) И НЕ (x > 44).", true),
    Output = "16"
}

Test! {
    Name = problem16011,
    Input = (NumberToFind::Max, None, "(x < 7) И НЕ (x < 6).", true),
    Output = "6"
}

Test! {
    Name = problem18171,
    Input = (NumberToFind::Max, None, "НЕ(Число > 10000) И (Число нечетное).", true),
    Output = "9999"
}

Test! {
    Name = problem18227,
    Input = (NumberToFind::Max, Some(2), "(первая цифра нечетная) И НЕ (число делится на 3).", true),
    Output = "98"
}

Test! {
    Name = problem10632,
    Input = (NumberToFind::Min, None, "НЕ (X < 2) И (X < 5).", true),
    Output = "2"
}

Test! {
    Name = problem10633,
    Input = (NumberToFind::Min, None, "НЕ (X < 2) И (X четное).", true),
    Output = "2"
}

Test! {
    Name = problem10634,
    Input = (NumberToFind::Min, None, "НЕ (X < 7) И (X четное).", true),
    Output = "8"
}

Test! {
    Name = problem10635,
    Input = (NumberToFind::Min, None, "НЕ (X < 6) И (X нечетное).", true),
    Output = "7"
}

Test! {
    Name = problem10636,
    Input = (NumberToFind::Min, None, "НЕ (X < 9) И НЕ (X нечетное).", true),
    Output = "10"
}

Test! {
    Name = problem10637,
    Input = (NumberToFind::Min, None, "НЕ (X <= 7) И (X < 20).", true),
    Output = "8"
}

Test! {
    Name = problem10870,
    Input = (NumberToFind::Min, None, "(X > 2) И НЕ (X > 13).", true),
    Output = "3"
}

Test! {
    Name = problem10871,
    Input = (NumberToFind::Min, None, "НЕ (X < 2) И НЕ (X > 10).", true),
    Output = "2"
}

Test! {
    Name = problem10872,
    Input = (NumberToFind::Min, None, "НЕ (X < 5) И НЕ (X > 9).", true),
    Output = "5"
}

Test! {
    Name = problem10877,
    Input = (NumberToFind::Min, None, "НЕ (X <= 8) И НЕ (X >= 15) И (X четное).", true),
    Output = "10"
}

Test! {
    Name = problem18033,
    Input = (NumberToFind::Min, Some(3), "НЕ (Число нечетное) И (Число кратно 3).", true),
    Output = "102"
}

Test! {
    Name = problem18186,
    Input = (NumberToFind::Min, None, "НЕ(Число < 100) И НЕ (Число нечетное)", true),
    Output = "100"
}

Test! {
    Name = problem18212,
    Input = (NumberToFind::Min, Some(2), "НЕ (первая цифра нечетная) И (число делится на 3).", true),
    Output = "21"
}

Test! {
    Name = problem18424,
    Input = (NumberToFind::Min, None, "(X > 16) И НЕ (X нечетное).", true),
    Output = "18"
}

Test! {
    Name = problem18242,
    Input = (NumberToFind::Min, None, "(X < 8) И НЕ (X < 7).", true),
    Output = "7"
}

Test! {
    Name = problem18257,
    Input = (NumberToFind::Min, None, "НЕ (X < 6) И (X < 7).", true),
    Output = "6"
}

Test! {
    Name = problem18272,
    Input = (NumberToFind::Min, None, "(X > 7) ИЛИ НЕ (X > 6).", false),
    Output = "7"
}

Test! {
    Name = problem18287,
    Input = (NumberToFind::Min, None, "(X > 3) ИЛИ НЕ (X > 2).", false),
    Output = "3"
}
