extern crate rusty_oge;

use std::collections::HashMap;

use rusty_oge::module7::*;

#[macro_use]
mod test_macros;

fn to_ip_parts(parts: &[(char, &str)]) -> HashMap<char, String> {
    HashMap::from_iter(parts.iter().map(|(k, v)| (k.to_owned(), v.to_string())))
}

Test! {
    Name = problem458,
    Input = (to_ip_parts(&[('А', "2.17"), ('Б', "16"), ('В', ".65"), ('Г', "8.121")])),
    Output = "БАГВ"
}

Test! {
    Name = problem478,
    Input = (to_ip_parts(&[('А', "4.243"), ('Б', "116.2"), ('В', "13"), ('Г', ".23")])),
    Output = "БВГА"
}

Test! {
    Name = problem498,
    Input = (to_ip_parts(&[('А', "2.12"), ('Б', "22"), ('В', ".30"), ('Г', "5.121")])),
    Output = "БАГВ"
}

Test! {
    Name = problem518,
    Input = (to_ip_parts(&[('А', "17"), ('Б', ".44"), ('В', "4.144"), ('Г', "9.13")])),
    Output = "АГВБ"
}

Test! {
    Name = problem538,
    Input = (to_ip_parts(&[('А', ".33"), ('Б', "3.232"), ('В', "3.20"), ('Г', "23")])),
    Output = "ГВБА"
}
