extern crate rusty_oge;

use std::collections::HashMap;

use rusty_oge::module4::*;

#[macro_use]
mod test_macros;

fn to_map(raw_map: &[(&str, &[(&str, usize)])]) -> HashMap<String, Vec<(String, usize)>> {
    let mut hash_map = HashMap::new();
    for (node_name, raw_neighbors) in raw_map {
        let neighbors = Vec::from_iter(raw_neighbors.iter().map(|(s, l)| (s.to_string(), *l)));
        hash_map.insert(node_name.to_string(), neighbors.to_vec());
    }
    hash_map
}

Test! {
    Name = problem3,
    Input = (to_map(&[
        ("A", &[("B", 1)]),
        ("B", &[("A", 1), ("C", 2), ("D", 2), ("E", 7)]),
        ("C", &[("B", 2), ("E", 3)]),
        ("D", &[("B", 2), ("E", 4)]),
        ("E", &[("B", 7), ("C", 3), ("D", 4)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem23,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 3)]),
        ("B", &[("A", 5), ("C", 1), ("D", 4)]),
        ("C", &[("A", 3), ("B", 1), ("D", 6)]),
        ("D", &[("B", 4), ("C", 6), ("E", 1)]),
        ("E", &[("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "9"
}

Test! {
    Name = problem43,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 7)]),
        ("B", &[("A", 3), ("C", 2), ("E", 8)]),
        ("C", &[("A", 7), ("B", 2), ("D", 4)]),
        ("D", &[("C", 4), ("E", 1)]),
        ("E", &[("B", 8), ("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "10"
}

Test! {
    Name = problem63,
    Input = (to_map(&[
        ("A", &[("B", 1)]),
        ("B", &[("A", 1), ("C", 4), ("D", 2), ("E", 8)]),
        ("C", &[("B", 4), ("E", 4)]),
        ("D", &[("B", 2), ("E", 4)]),
        ("E", &[("B", 8), ("C", 4), ("D", 4)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem83,
    Input = (to_map(&[
        ("A", &[("B", 4), ("C", 7)]),
        ("B", &[("A", 4), ("C", 1), ("D", 5)]),
        ("C", &[("A", 7), ("B", 1), ("D", 3)]),
        ("D", &[("B", 5), ("C", 3), ("E", 1)]),
        ("E", &[("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "9"
}

Test! {
    Name = problem103,
    Input = (to_map(&[
        ("A", &[("B", 7), ("C", 4)]),
        ("B", &[("A", 7), ("C", 2), ("E", 4)]),
        ("C", &[("A", 4), ("B", 2), ("D", 4)]),
        ("D", &[("C", 4), ("E", 4)]),
        ("E", &[("B", 4), ("D", 4)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "10"
}

Test! {
    Name = problem123,
    Input = (to_map(&[
        ("A", &[("B", 3)]),
        ("B", &[("A", 3), ("C", 1), ("D", 2), ("E", 6)]),
        ("C", &[("B", 1), ("E", 3)]),
        ("D", &[("B", 2), ("E", 3)]),
        ("E", &[("B", 6), ("C", 3), ("D", 3)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem143,
    Input = (to_map(&[
        ("A", &[("B", 2), ("D", 1)]),
        ("B", &[("A", 2), ("C", 3), ("D", 3)]),
        ("C", &[("B", 3), ("D", 3), ("E", 2)]),
        ("D", &[("A", 1), ("B", 3), ("C", 3)]),
        ("E", &[("C", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem183,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 3)]),
        ("B", &[("A", 2), ("D", 3), ("E", 5)]),
        ("C", &[("A", 3), ("D", 4)]),
        ("D", &[("B", 3), ("C", 4), ("E", 1)]),
        ("E", &[("B", 5), ("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem203,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 6), ("D", 4)]),
        ("B", &[("A", 2), ("C", 3)]),
        ("C", &[("A", 6), ("B", 3), ("D", 3), ("E", 2)]),
        ("D", &[("A", 4), ("C", 3)]),
        ("E", &[("C", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem223,
    Input = (to_map(&[
        ("A", &[("B", 2), ("D", 4)]),
        ("B", &[("A", 2), ("C", 5), ("D", 1)]),
        ("C", &[("B", 5), ("D", 3), ("E", 2)]),
        ("D", &[("A", 4), ("B", 1), ("C", 3)]),
        ("E", &[("C", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem243,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 3)]),
        ("B", &[("A", 3), ("D", 5), ("E", 6)]),
        ("C", &[("A", 3), ("D", 4)]),
        ("D", &[("B", 4), ("C", 4), ("E", 1)]),
        ("E", &[("B", 6), ("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem263,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 5), ("D", 1)]),
        ("B", &[("A", 2), ("C", 1)]),
        ("C", &[("A", 5), ("B", 1), ("D", 3), ("E", 2)]),
        ("D", &[("A", 1), ("C", 3)]),
        ("E", &[("C", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem283,
    Input = (to_map(&[
        ("A", &[("B", 4), ("C", 4)]),
        ("B", &[("A", 4), ("C", 1), ("D", 5)]),
        ("C", &[("A", 4), ("B", 1), ("D", 3)]),
        ("D", &[("B", 5), ("C", 3), ("E", 1)]),
        ("E", &[("D", 1)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem303,
    Input = (to_map(&[
        ("A", &[("B", 7), ("C", 4)]),
        ("B", &[("A", 7), ("C", 2), ("E", 5)]),
        ("C", &[("A", 4), ("B", 2), ("D", 4)]),
        ("D", &[("C", 4), ("E", 5)]),
        ("E", &[("B", 5), ("D", 5)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "11"
}

Test! {
    Name = problem323,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5), ("F", 15)]),
        ("B", &[("A", 3), ("C", 3)]),
        ("C", &[("A", 5), ("B", 3), ("D", 5), ("E", 2)]),
        ("D", &[("C", 5), ("F", 3)]),
        ("E", &[("C", 2), ("F", 7)]),
        ("F", &[("A", 15), ("D", 3), ("E", 7)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "13"
}

Test! {
    Name = problem344,
    Input = (to_map(&[
        ("A", &[("B", 6), ("C", 4), ("D", 2), ("E", 1)]),
        ("B", &[("A", 6), ("C", 1)]),
        ("C", &[("A", 4), ("B", 1), ("D", 3), ("F", 1)]),
        ("D", &[("A", 2), ("C", 3), ("E", 1)]),
        ("E", &[("A", 1), ("D", 1), ("F", 6)]),
        ("F", &[("C", 1), ("E", 6)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem364,
    Input = (to_map(&[
        ("A", &[("B", 6), ("C", 4), ("D", 2), ("E", 1)]),
        ("B", &[("A", 6), ("C", 1)]),
        ("C", &[("A", 4), ("B", 1), ("D", 3), ("F", 2)]),
        ("D", &[("A", 2), ("C", 3), ("E", 2)]),
        ("E", &[("A", 1), ("D", 2), ("F", 6)]),
        ("F", &[("C", 2), ("E", 6)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem384,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 5), ("D", 4)]),
        ("B", &[("A", 5), ("C", 2)]),
        ("C", &[("A", 5), ("B", 2), ("F", 1)]),
        ("D", &[("A", 4), ("E", 1), ("F", 3)]),
        ("E", &[("D", 1), ("F", 1)]),
        ("F", &[("C", 1), ("D", 3), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem404,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 5), ("D", 4)]),
        ("B", &[("A", 5), ("C", 2)]),
        ("C", &[("A", 5), ("B", 2), ("F", 2)]),
        ("D", &[("A", 4), ("E", 2), ("F", 3)]),
        ("E", &[("D", 2), ("F", 1)]),
        ("F", &[("C", 2), ("D", 3), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem424,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 5), ("F", 9)]),
        ("B", &[("A", 2), ("C", 2), ("D", 1), ("F", 5)]),
        ("C", &[("A", 5), ("B", 2), ("E", 1)]),
        ("D", &[("B", 1)]),
        ("E", &[("C", 1), ("F", 1)]),
        ("F", &[("A", 9), ("B", 5), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem444,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 5), ("F", 7)]),
        ("B", &[("A", 2), ("C", 2), ("D", 1), ("F", 5)]),
        ("C", &[("A", 5), ("B", 2), ("E", 1)]),
        ("D", &[("B", 1)]),
        ("E", &[("C", 1), ("F", 2)]),
        ("F", &[("F", 7), ("B", 5), ("E", 2)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem464,
    Input = (to_map(&[
        ("A", &[("B", 7), ("C", 2), ("D", 2), ("E", 5), ("F", 5)]),
        ("B", &[("A", 7), ("C", 2)]),
        ("C", &[("A", 2), ("B", 2), ("D", 1)]),
        ("D", &[("A", 2), ("C", 1), ("E", 1)]),
        ("E", &[("A", 5), ("D", 1), ("F", 1)]),
        ("F", &[("A", 5), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "4"
}

Test! {
    Name = problem484,
    Input = (to_map(&[
        ("A", &[("B", 7), ("C", 2), ("D", 2), ("E", 5), ("F", 5)]),
        ("B", &[("A", 7), ("C", 2)]),
        ("C", &[("A", 2), ("B", 2), ("D", 1)]),
        ("D", &[("A", 2), ("C", 1), ("E", 1)]),
        ("E", &[("A", 5), ("D", 2), ("F", 2)]),
        ("F", &[("A", 5), ("E", 2)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem504,
    Input = (to_map(&[
        ("A", &[("C", 2), ("D", 1)]),
        ("B", &[("C", 1), ("F", 3)]),
        ("C", &[("A", 2), ("B", 1), ("F", 4)]),
        ("D", &[("A", 1), ("E", 1), ("F", 4)]),
        ("E", &[("D", 1), ("F", 5)]),
        ("F", &[("B", 3), ("C", 4), ("D", 4), ("E", 5)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem524,
    Input = (to_map(&[
        ("A", &[("C", 2), ("D", 1)]),
        ("B", &[("C", 1), ("F", 3)]),
        ("C", &[("A", 2), ("B", 1), ("F", 6)]),
        ("D", &[("A", 1), ("E", 1), ("F", 6)]),
        ("E", &[("D", 1), ("F", 5)]),
        ("F", &[("B", 3), ("C", 6), ("D", 6), ("E", 5)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem624,
    Input = (to_map(&[
        ("A", &[("V", 1), ("E", 1)]),
        ("V", &[("A", 1), ("D", 5)]),
        ("C", &[("D", 1), ("E", 2)]),
        ("D", &[("V", 5), ("C", 1), ("E", 7)]),
        ("E", &[("A", 1), ("C", 2), ("D", 7)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "4"
}

Test! {
    Name = problem644,
    Input = (to_map(&[
        ("A", &[("V", 1), ("E", 2)]),
        ("V", &[("A", 1), ("C", 7)]),
        ("C", &[("V", 7), ("D", 1), ("E", 2)]),
        ("D", &[("C", 1), ("E", 6)]),
        ("E", &[("A", 2), ("C", 2), ("D", 6)]),
    ]), ("V", "D"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem664,
    Input = (to_map(&[
        ("B", &[("V", 5), ("K", 8), ("D1", 10), ("I", 9)]),
        ("V", &[("B", 5), ("D1", 4), ("D2", 2)]),
        ("K", &[("B", 8), ("D2", 1), ("I", 3)]),
        ("D1", &[("B", 10), ("V", 4), ("I", 5)]),
        ("D2", &[("V", 2), ("K", 1)]),
        ("I", &[("B", 9), ("K", 3), ("D1", 5)]),
    ]), ("V", "I"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem684,
    Input = (to_map(&[
        ("B", &[("V", 5), ("K", 8), ("D1", 10), ("I", 9)]),
        ("V", &[("B", 5), ("D2", 2)]),
        ("K", &[("B", 8), ("D1", 5), ("D2", 1), ("I", 3)]),
        ("D1", &[("B", 10), ("K", 5), ("D2", 7), ("I", 5)]),
        ("D2", &[("V", 2), ("K", 1), ("D1", 7)]),
        ("I", &[("B", 9), ("K", 3), ("D1", 5)]),
    ]), ("D1", "V"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem704,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 5), ("D", 1)]),
        ("B", &[("A", 2), ("C", 1)]),
        ("C", &[("A", 5), ("B", 1), ("D", 3), ("E", 2)]),
        ("D", &[("A", 1), ("C", 3)]),
        ("E", &[("C", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem799,
    Input = (to_map(&[
        ("A", &[("B", 12), ("C", 2), ("D", 5), ("E", 1)]),
        ("B", &[("A", 12), ("C", 8), ("D", 1), ("E", 5)]),
        ("C", &[("A", 2), ("B", 8), ("D", 1)]),
        ("D", &[("A", 5), ("B", 1), ("C", 1)]),
        ("E", &[("A", 1), ("B", 5)]),
    ]), ("A", "B"), vec![], PathToFind::Shortest),
    Output = "4"
}

Test! {
    Name = problem819,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 1), ("D", 3)]),
        ("B", &[("A", 5), ("D", 4), ("E", 1)]),
        ("C", &[("A", 1), ("E", 1)]),
        ("D", &[("A", 3), ("B", 4), ("E", 1)]),
        ("E", &[("B", 1), ("C", 5), ("D", 1)]),
    ]), ("A", "B"), vec![], PathToFind::Shortest),
    Output = "3"
}

Test! {
    Name = problem841,
    Input = (to_map(&[
        ("A", &[("C", 8), ("D", 10)]),
        ("B", &[("D", 4), ("E", 1)]),
        ("C", &[("A", 8), ("D", 1), ("E", 3)]),
        ("D", &[("A", 10), ("B", 4), ("C", 1)]),
        ("E", &[("B", 1), ("C", 3)]),
    ]), ("A", "B"), vec![], PathToFind::Shortest),
    Output = "12"
}

Test! {
    Name = problem861,
    Input = (to_map(&[
        ("A", &[("B", 2), ("D", 8)]),
        ("B", &[("A", 2), ("D", 4), ("E", 1)]),
        ("C", &[("D", 1), ("E", 3)]),
        ("D", &[("A", 8), ("B", 4), ("C", 1), ("E", 5)]),
        ("E", &[("B", 1), ("C", 3), ("D", 5)]),
    ]), ("A", "C"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem882,
    Input = (to_map(&[
        ("A", &[("B", 4), ("D", 8), ("E", 3)]),
        ("B", &[("A", 4), ("C", 1)]),
        ("C", &[("B", 1), ("D", 2)]),
        ("D", &[("A", 8), ("C", 2), ("E", 3)]),
        ("E", &[("A", 3), ("D", 3)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem902,
    Input = (to_map(&[
        ("A", &[("B", 2), ("D", 6)]),
        ("B", &[("A", 2), ("C", 2), ("D", 8)]),
        ("C", &[("B", 2), ("E", 2)]),
        ("D", &[("A", 6), ("B", 8), ("E", 2)]),
        ("E", &[("C", 2), ("D", 2)]),
    ]), ("A", "E"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem922,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 8), ("D", 3)]),
        ("B", &[("A", 2), ("C", 7)]),
        ("C", &[("A", 8), ("B", 7), ("D", 9), ("E", 1)]),
        ("D", &[("A", 3), ("C", 9), ("E", 1)]),
        ("E", &[("C", 1), ("D", 1)]),
    ]), ("A", "C"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem942,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 9), ("D", 5)]),
        ("B", &[("A", 3), ("C", 6)]),
        ("C", &[("A", 9), ("B", 6), ("D", 3), ("E", 1)]),
        ("D", &[("A", 5), ("C", 3), ("E", 1)]),
        ("E", &[("C", 1), ("D", 1)]),
    ]), ("A", "C"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem1097,
    Input = (to_map(&[
        ("A", &[("B", 1), ("C", 5), ("E", 2)]),
        ("B", &[("A", 1), ("D", 6)]),
        ("C", &[("A", 5), ("D", 1), ("E", 7)]),
        ("D", &[("B", 6), ("C", 1)]),
        ("E", &[("A", 2), ("C", 7)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem1117,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 1), ("E", 5)]),
        ("B", &[("A", 2), ("C", 4)]),
        ("C", &[("A", 1), ("B", 4), ("D", 1), ("E", 4)]),
        ("D", &[("C", 1), ("E", 2)]),
        ("E", &[("A", 5), ("C", 4), ("D", 2)]),
    ]), ("B", "E"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem1137,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 6), ("D", 10), ("E", 5)]),
        ("B", &[("A", 5), ("D", 4)]),
        ("C", &[("A", 6), ("D", 2), ("E", 7)]),
        ("D", &[("A", 10), ("B", 4), ("C", 2), ("E", 5)]),
        ("E", &[("A", 5), ("C", 7), ("D", 5)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem1157,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 7), ("D", 4)]),
        ("B", &[("A", 2), ("C", 5), ("D", 1)]),
        ("C", &[("A", 7), ("B", 5), ("D", 2)]),
        ("D", &[("A", 4), ("B", 1), ("C", 2)]),
    ]), ("A", "C"), vec![], PathToFind::Shortest),
    Output = "5"
}

Test! {
    Name = problem1216,
    Input = (to_map(&[
        ("A", &[("B", 8), ("C", 3)]),
        ("B", &[("A", 8), ("D", 3)]),
        ("C", &[("A", 3), ("E", 4), ("F", 3)]),
        ("D", &[("B", 3), ("E", 1), ("F", 3)]),
        ("E", &[("C", 4), ("D", 1), ("F", 2)]),
        ("F", &[("C", 3), ("D", 3), ("E", 2)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem1256,
    Input = (to_map(&[
        ("A", &[("B", 6), ("D", 1), ("E", 4)]),
        ("B", &[("A", 6), ("C", 2), ("D", 5)]),
        ("C", &[("B", 2), ("D", 2)]),
        ("D", &[("A", 1), ("B", 5), ("C", 2), ("E", 6)]),
        ("E", &[("A", 4), ("D", 6)]),
    ]), ("B", "E"), vec![], PathToFind::Shortest),
    Output = "9"
}

Test! {
    Name = problem4500,
    Input = (to_map(&[
        ("A", &[("B", 6), ("E", 3)]),
        ("B", &[("A", 6), ("C", 2), ("D", 5), ("E", 1)]),
        ("C", &[("B", 2), ("D", 2)]),
        ("D", &[("B", 5), ("C", 2), ("E", 6)]),
        ("E", &[("A", 3), ("B", 1), ("D", 6)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem4556,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5)]),
        ("B", &[("A", 3), ("C", 1), ("D", 6)]),
        ("C", &[("A", 5), ("B", 1), ("D", 4), ("E", 1)]),
        ("D", &[("B", 6), ("C", 4), ("E", 3)]),
        ("E", &[("C", 1), ("D", 3)]),
    ]), ("A", "D"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem4564,
    Input = (to_map(&[
        ("A", &[("B", 1), ("C", 5), ("F", 15)]),
        ("B", &[("A", 1), ("C", 2)]),
        ("C", &[("A", 5), ("B", 2), ("D", 1)]),
        ("D", &[("C", 1), ("E", 2), ("F", 6)]),
        ("E", &[("D", 2), ("F", 1)]),
        ("F", &[("A", 15), ("D", 6), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem5500,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 7), ("F", 2)]),
        ("B", &[("A", 3)]),
        ("C", &[("A", 7), ("D", 3), ("E", 1)]),
        ("D", &[("C", 3), ("E", 1), ("F", 2)]),
        ("E", &[("C", 1), ("D", 1)]),
        ("F", &[("A", 2), ("D", 2)]),
    ]), ("B", "C"), vec![], PathToFind::Shortest),
    Output = "9"
}

Test! {
    Name = problem5797,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 8), ("D", 3)]),
        ("B", &[("A", 5), ("C", 2), ("D", 1)]),
        ("C", &[("A", 8), ("B", 2), ("D", 4)]),
        ("D", &[("A", 3), ("B", 1), ("C", 4)]),
    ]), ("A", "C"), vec![], PathToFind::Shortest),
    Output = "6"
}

Test! {
    Name = problem12853,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 9), ("D", 4)]),
        ("B", &[("A", 2), ("C", 3), ("E", 5)]),
        ("C", &[("A", 9), ("B", 3), ("D", 6), ("E", 10)]),
        ("D", &[("A", 4), ("C", 6), ("E", 8)]),
        ("E", &[("B", 5), ("C", 10), ("D", 8)]),
    ]), ("A", "E"), vec!["C".into()], PathToFind::Shortest),
    Output = "15"
}

Test! {
    Name = problem16012,
    Input = (to_map(&[
        ("A", &[("B", 2), ("E", 1)]),
        ("B", &[("A", 2), ("C", 5), ("E", 4)]),
        ("C", &[("B", 5), ("D", 3), ("E", 3)]),
        ("D", &[("C", 3), ("E", 4)]),
        ("E", &[("A", 1), ("B", 4), ("C", 3), ("D", 4)]),
    ]), ("B", "D"), vec![], PathToFind::Shortest),
    Output = "7"
}

Test! {
    Name = problem18034,
    Input = (to_map(&[
        ("A", &[("B", 1), ("C", 2), ("E", 4)]),
        ("B", &[("A", 1), ("C", 4), ("E", 4)]),
        ("C", &[("A", 2), ("B", 4), ("E", 1)]),
        ("D", &[("E", 4)]),
        ("E", &[("A", 4), ("C", 1), ("D", 4)]),
    ]), ("B", "D"), vec![], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem18172,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 4), ("F", 15)]),
        ("B", &[("A", 3), ("C", 3), ("D", 4)]),
        ("C", &[("A", 4), ("B", 3), ("D", 1), ("F", 6)]),
        ("D", &[("B", 4), ("C", 1), ("E", 2), ("F", 6)]),
        ("E", &[("D", 2), ("F", 1)]),
        ("F", &[("A", 15), ("C", 6), ("D", 6), ("E", 1)]),
    ]), ("A", "F"), vec!["C".into()], PathToFind::Shortest),
    Output = "8"
}

Test! {
    Name = problem18187,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5), ("F", 15)]),
        ("B", &[("A", 3), ("C", 1), ("D", 4)]),
        ("C", &[("A", 5), ("B", 1), ("D", 2), ("F", 9)]),
        ("D", &[("E", 3), ("F", 6)]),
        ("E", &[("D", 3), ("F", 4)]),
        ("F", &[("A", 15), ("C", 9), ("D", 6), ("E", 4)]),
    ]), ("A", "F"), vec!["C".into()], PathToFind::Shortest),
    Output = "12"
}

Test! {
    Name = problem18213,
    Input = (to_map(&[
        ("A", &[("B", 5), ("C", 9), ("D", 6), ("E", 9)]),
        ("B", &[("A", 5), ("D", 4)]),
        ("C", &[("A", 9), ("D", 2), ("E", 2)]),
        ("D", &[("A", 6), ("B", 4), ("C", 2), ("E", 5)]),
        ("E", &[("A", 9), ("C", 2), ("D", 5)]),
    ]), ("A", "E"), vec!["D".into()], PathToFind::Shortest),
    Output = "10"
}

Test! {
    Name = problem18228,
    Input = (to_map(&[
        ("A", &[("B", 2), ("C", 1), ("E", 1)]),
        ("B", &[("A", 2), ("D", 1)]),
        ("C", &[("A", 1), ("D", 2), ("E", 2)]),
        ("D", &[("B", 1), ("C", 2), ("E", 1)]),
        ("E", &[("A", 1), ("C", 2), ("D", 1)]),
    ]), ("A", "B"), vec!["D".into()], PathToFind::Shortest),
    Output = "3"
}

Test! {
    Name = problem18243,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 4), ("F", 18)]),
        ("B", &[("A", 3), ("C", 3)]),
        ("C", &[("A", 4), ("B", 3), ("D", 4)]),
        ("D", &[("C", 4), ("E", 2), ("F", 6)]),
        ("E", &[("D", 2), ("F", 1)]),
        ("F", &[("A", 18), ("D", 6), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "11"
}

Test! {
    Name = problem18258,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5), ("F", 15)]),
        ("B", &[("A", 3), ("C", 4)]),
        ("C", &[("A", 5), ("D", 2)]),
        ("D", &[("B", 4), ("C", 2), ("E", 3), ("F", 6)]),
        ("E", &[("D", 3), ("F", 4)]),
        ("F", &[("A", 15), ("D", 6), ("E", 4)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "13"
}

Test! {
    Name = problem18273,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5), ("F", 15)]),
        ("B", &[("A", 3), ("C", 1)]),
        ("C", &[("A", 5), ("B", 1), ("D", 1)]),
        ("D", &[("C", 1), ("E", 2), ("F", 6)]),
        ("E", &[("D", 2), ("F", 2)]),
        ("F", &[("A", 15), ("D", 6), ("E", 2)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "9"
}

Test! {
    Name = problem18288,
    Input = (to_map(&[
        ("A", &[("B", 3), ("C", 5), ("F", 15)]),
        ("B", &[("A", 3), ("C", 1)]),
        ("C", &[("A", 5), ("B", 1), ("D", 2)]),
        ("D", &[("C", 2), ("E", 4), ("F", 6)]),
        ("E", &[("D", 4), ("F", 1)]),
        ("F", &[("A", 15), ("D", 6), ("E", 1)]),
    ]), ("A", "F"), vec![], PathToFind::Shortest),
    Output = "11"
}

Test! {
    Name = problem18425,
    Input = (to_map(&[
        ("A", &[("B", 1), ("C", 4), ("D", 3), ("E", 7)]),
        ("B", &[("A", 1), ("C", 2), ("D", 5)]),
        ("C", &[("A", 4), ("B", 2), ("D", 3)]),
        ("D", &[("A", 3), ("B", 5), ("C", 3), ("E", 2)]),
        ("E", &[("A", 7), ("D", 2)]),
    ]), ("A", "E"), vec!["C".into()], PathToFind::Shortest),
    Output = "8"
}
