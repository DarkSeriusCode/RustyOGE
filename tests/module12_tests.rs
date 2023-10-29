extern crate rusty_oge;

use std::env;
use std::path::PathBuf;

use rusty_oge::module12::*;

#[macro_use]
mod test_macros;

fn path_to(fname: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(env::current_dir().unwrap());
    buf.push("tests/module12_files");
    buf.push(fname);
    buf
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem10504,
    Input = (&path_to("10504-10601.rar"), "Проза"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "16"
}

Test! {
    Name = problem10507,
    Input = (&path_to("10504-10601.rar"), "Проза"),
    Spec = ProblemSpec::WithExtencions(vec!["pdf".into()]),
    Output = "11"
}

Test! {
    Name = problem10510,
    Input = (&path_to("10504-10601.rar"), "Проза"),
    Spec = ProblemSpec::WithExtencions(vec!["htm".into()]),
    Output = "10"
}

Test! {
    Name = problem10512,
    Input = (&path_to("10504-10601.rar"), "Проза"),
    Spec = ProblemSpec::WithExtencions(vec!["rtf".into()]),
    Output = "14"
}

Test! {
    Name = problem10514,
    Input = (&path_to("10504-10601.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["rtf".into()]),
    Output = "11"
}

Test! {
    Name = problem10516,
    Input = (&path_to("10504-10601.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["pdf".into()]),
    Output = "16"
}

Test! {
    Name = problem10518,
    Input = (&path_to("10504-10601.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "12"
}

Test! {
    Name = problem10522,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "33"
}

Test! {
    Name = problem10524,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["rtf".into()]),
    Output = "25"
}

Test! {
    Name = problem10596,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["pdf".into()]),
    Output = "31"
}

Test! {
    Name = problem10597,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["htm".into()]),
    Output = "17"
}

Test! {
    Name = problem10598,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["html".into()]),
    Output = "3"
}

Test! {
    Name = problem10599,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["odt".into()]),
    Output = "2"
}

Test! {
    Name = problem10600,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["docx".into()]),
    Output = "3"
}

Test! {
    Name = problem10601,
    Input = (&path_to("10504-10601.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencions(vec!["doc".into(), "docx".into()]),
    Output = "4"
}

Test! {
    Name = problem11322,
    Input = (&path_to("11322.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["doc".into()]),
    Output = "3"
}

Test! {
    Name = problem11323,
    Input = (&path_to("11323.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["docx".into()]),
    Output = "2"
}

Test! {
    Name = problem11324,
    Input = (&path_to("11324.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["doc".into(), "docx".into()]),
    Output = "5"
}

Test! {
    Name = problem11325,
    Input = (&path_to("11325.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "12"
}

Test! {
    Name = problem11326,
    Input = (&path_to("11326.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["rtf".into()]),
    Output = "10"
}

Test! {
    Name = problem11327,
    Input = (&path_to("11327.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["pdf".into()]),
    Output = "7"
}

Test! {
    Name = problem11328,
    Input = (&path_to("11328.rar"), "Task12"),
    Spec = ProblemSpec::WithExtencions(vec!["htm".into()]),
    Output = "3"
}

Test! {
    Name = problem11329,
    Input = (&path_to("11329.rar"), "Files"),
    Spec = ProblemSpec::WithExtencions(vec!["py".into()]),
    Output = "4"
}

Test! {
    Name = problem11330,
    Input = (&path_to("11330.rar"), "Files"),
    Spec = ProblemSpec::WithExtencions(vec!["js".into()]),
    Output = "12"
}

Test! {
    Name = problem11331,
    Input = (&path_to("11331.rar"), "Files"),
    Spec = ProblemSpec::WithExtencions(vec!["html".into()]),
    Output = "5"
}

Test! {
    Name = problem18180,
    Input = (&path_to("18180.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["odt".into()]),
    Output = "20"
}

Test! {
    Name = problem18195,
    Input = (&path_to("18195.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["docx".into()]),
    Output = "27"
}

Test! {
    Name = problem18221,
    Input = (&path_to("18221.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["pdf".into()]),
    Output = "17"
}

Test! {
    Name = problem18236,
    Input = (&path_to("18236.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "13"
}

Test! {
    Name = problem18251,
    Input = (&path_to("18251.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["txt".into()]),
    Output = "12"
}

Test! {
    Name = problem18266,
    Input = (&path_to("18266.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["htm".into()]),
    Output = "7"
}

Test! {
    Name = problem18281,
    Input = (&path_to("18281.rar"), "11-12 Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["rtf".into()]),
    Output = "15"
}

Test! {
    Name = problem18296,
    Input = (&path_to("18296.rar"), "Поэзия"),
    Spec = ProblemSpec::WithExtencions(vec!["html".into()]),
    Output = "13"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem16020,
    Input = (&path_to("16020.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencionAndSize("htm".into(), FileSize::Kb(1024)),
    Output = "3"
}

Test! {
    Name = problem18042,
    Input = (&path_to("18042.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencionAndSize("rtf".into(), FileSize::Mb(2)),
    Output = "5"
}

Test! {
    Name = problem18762,
    Input = (&path_to("18762.rar"), "DEMO-12"),
    Spec = ProblemSpec::WithExtencionAndSize("rtf".into(), FileSize::Mb(2)),
    Output = "5"
}
