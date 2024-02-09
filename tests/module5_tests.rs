extern crate rusty_oge;

use std::collections::HashMap;
use std::str::FromStr;

use rusty_oge::module5::*;

#[macro_use]
mod test_macros;

fn to_cmd_table(cmds: &[(char, &str)]) -> CommandTable {
    let map_closure = |item: &(char, &str)| {
        (
            item.0.to_owned(),
            Command::from_str(item.1).expect("Cannot create Command from string!")
        )

    };
    HashMap::from_iter(cmds.iter().map(map_closure))
}

Test! {
    Name = problem10383,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "*b")]), 6, 82),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "10"
}

Test! {
    Name = problem10384,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "*b")]), 3, 62),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "12"
}

Test! {
    Name = problem10385,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "*b")]), 7, 51),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "5"
}

Test! {
    Name = problem10386,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "*b")]), 4, 72),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "7"
}

Test! {
    Name = problem10387,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "*b")]), 1, 97),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "13"
}

Test! {
    Name = problem10388,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "*b")]), 3, 75),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "6"
}

Test! {
    Name = problem10389,
    Input = (to_cmd_table(&[('1', "+4"), ('2', "*b")]), 9, 77),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "5"
}

Test! {
    Name = problem10878,
    Input = (to_cmd_table(&[('1', "+5"), ('2', "*b")]), 6, 48),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "3"
}

Test! {
    Name = problem10879,
    Input = (to_cmd_table(&[('1', "+5"), ('2', "*b")]), 3, 88),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "6"
}

Test! {
    Name = problem10880,
    Input = (to_cmd_table(&[('1', "+5"), ('2', "*b")]), 4, 81),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "4"
}

Test! {
    Name = problem10881,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "*b")]), 4, 202),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "20"
}

Test! {
    Name = problem10882,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "*b")]), 6, 164),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "16"
}

Test! {
    Name = problem12854,
    Input = (to_cmd_table(&[('1', "-b"), ('2', "*5")]), 2, 17),
    Spec = ProblemSpec::FindVariableValue("21121".into()),
    Output = "3"
}

Test! {
    Name = problem16013,
    Input = (to_cmd_table(&[('1', "^2"), ('2', "+3")]), 1, 25),
    Spec = ProblemSpec::MakeAlgorithm(5),
    Output = "21222"
}

Test! {
    Name = problem18035,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "^2")]), 3, 84),
    Spec = ProblemSpec::MakeAlgorithm(5),
    Output = "22111"
}

Test! {
    Name = problem18173,
    Input = (to_cmd_table(&[('1', "^2"), ('2', "+b")]), 2, 37),
    Spec = ProblemSpec::FindVariableValue("12212".into()),
    Output = "1"
}

Test! {
    Name = problem18188,
    Input = (to_cmd_table(&[('1', "^2"), ('2', "+b")]), 2, 72),
    Spec = ProblemSpec::FindVariableValue("12122".into()),
    Output = "4"
}

Test! {
    Name = problem18214,
    Input = (to_cmd_table(&[('1', "*b"), ('2', "+2")]), 1, 91),
    Spec = ProblemSpec::FindVariableValue("12221".into()),
    Output = "7"
}

Test! {
    Name = problem18229,
    Input = (to_cmd_table(&[('1', "*b"), ('2', "+1")]), 1, 56),
    Spec = ProblemSpec::FindVariableValue("21212".into()),
    Output = "5"
}

Test! {
    Name = problem18244,
    Input = (to_cmd_table(&[('1', "*4"), ('2', "-b")]), 4, 28),
    Spec = ProblemSpec::FindVariableValue("21122".into()),
    Output = "2"
}

Test! {
    Name = problem18259,
    Input = (to_cmd_table(&[('1', "*4"), ('2', "-b")]), 3, 21),
    Spec = ProblemSpec::FindVariableValue("12212".into()),
    Output = "3"
}

Test! {
    Name = problem32095,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "*b")]), 6, 82),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "10"
}

Test! {
    Name = problem10390,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "/b")]), 63, 11),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "8"
}

Test! {
    Name = problem10391,
    Input = (to_cmd_table(&[('1', "+1"), ('2', "/b")]), 50, 20),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "3"
}

Test! {
    Name = problem10392,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "/b")]), 47, 13),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "7"
}

Test! {
    Name = problem10393,
    Input = (to_cmd_table(&[('1', "+2"), ('2', "/b")]), 50, 22),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "3"
}

Test! {
    Name = problem10454,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "/b")]), 30, 12),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "6"
}

Test! {
    Name = problem10455,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "/b")]), 63, 21),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "4"
}

Test! {
    Name = problem10456,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "/b")]), 30, 6),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "13"
}

Test! {
    Name = problem10457,
    Input = (to_cmd_table(&[('1', "+3"), ('2', "/b")]), 46, 8),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "11"
}

Test! {
    Name = problem10883,
    Input = (to_cmd_table(&[('1', "+4"), ('2', "/b")]), 76, 26),
    Spec = ProblemSpec::FindVariableValue("11121".into()),
    Output = "4"
}

Test! {
    Name = problem10884,
    Input = (to_cmd_table(&[('1', "+4"), ('2', "/b")]), 49, 27),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "3"
}

Test! {
    Name = problem10885,
    Input = (to_cmd_table(&[('1', "+4"), ('2', "/b")]), 41, 17),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "9"
}

Test! {
    Name = problem10886,
    Input = (to_cmd_table(&[('1', "+4"), ('2', "/b")]), 48, 16),
    Spec = ProblemSpec::FindVariableValue("12111".into()),
    Output = "13"
}

Test! {
    Name = problem10887,
    Input = (to_cmd_table(&[('1', "+5"), ('2', "/b")]), 40, 20),
    Spec = ProblemSpec::FindVariableValue("11211".into()),
    Output = "5"
}

Test! {
    Name = problem18274,
    Input = (to_cmd_table(&[('1', "/2"), ('2', "-3")]), 76, 5),
    Spec = ProblemSpec::MakeAlgorithm(5),
    Output = "11212"
}

Test! {
    Name = problem18289,
    Input = (to_cmd_table(&[('1', "/2"), ('2', "-1")]), 65, 4),
    Spec = ProblemSpec::MakeAlgorithm(5),
    Output = "21111"
}
