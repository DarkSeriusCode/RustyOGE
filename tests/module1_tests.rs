extern crate rusty_oge;

use rusty_oge::module1::*;
use rusty_oge::utils::data_size::DataSizeUnit;

#[macro_use]
mod test_macros;

Test! {
    Name = problem10313,
    Input = (8, InputText::ConcreteText("Обь, Лена, Волга, Москва, Макензи, Амазонка — реки".into())),
    Spec = ProblemSpec::FindWord(8),
    Output = "Москва"
}

Test! {
    Name = problem10314,
    Input = (8, InputText::ConcreteText("Чад, Куба, Катар, Швеция, Эстония, Танзания, Сальвадор — страны".into())),
    Spec = ProblemSpec::FindWord(11),
    Output = "Сальвадор"
}

Test! {
    Name = problem10315,
    Input = (8, InputText::ConcreteText("Аки, Бали, Банда, Сибуян, Камотес, Лабрадор, Линкольна  — моря".into())),
    Spec = ProblemSpec::FindWord(7),
    Output = "Банда"
}

Test! {
    Name = problem10316,
    Input = (8, InputText::ConcreteText("Ява, Куба, Лусон, Маражо, Суматра, Сулавеси, Эспаньола  — острова".into())),
    Spec = ProblemSpec::FindWord(9),
    Output = "Суматра"
}

Test! {
    Name = problem10317,
    Input = (8, InputText::ConcreteText("ерш, Щука, Бычок, Карась, Гимнура, Долгопер  — рыбы".into())),
    Spec = ProblemSpec::FindWord(10),
    Output = "Долгопер"
}

Test! {
    Name = problem10863,
    Input = (8, InputText::ConcreteText("J, Cg, Cat, Ruby, Swift, Delphi, Haskell  — языки программирования".into())),
    Spec = ProblemSpec::FindWord(9),
    Output = "Haskell"
}

Test! {
    Name = problem16009,
    Input = (8, InputText::ConcreteText("Школьные предметы: ОБЖ, химия, физика, алгебра, биология, география, литература, информатика".into())),
    Spec = ProblemSpec::FindWord(11),
    Output = "география"
}

Test! {
    Name = problem18031,
    Input = (8, InputText::ConcreteText("Скользя по утреннему снегу, Друг милый, предадимся бегу Нетерпеливого коня И навестим поля пустые...".into())),
    Spec = ProblemSpec::FindWord(8),
    Output = "Скользя"
}

Test! {
    Name = problem10238,
    Input = (16, InputText::ConcreteText("еж, лев, слон, олень, тюлень, носорог, крокодил, аллигатор  — дикие животные".into())),
    Spec = ProblemSpec::FindWord(16),
    Output = "тюлень"
}

Test! {
    Name = problem10307,
    Input = (16, InputText::ConcreteText("Ель, кедр, сосна, кипарис, лиственница, можжевельник  — хвойные растения".into())),
    Spec = ProblemSpec::FindWord(26),
    Output = "лиственница"
}

Test! {
    Name = problem10308,
    Input = (16, InputText::ConcreteText("Лев, тигр, ягуар, гепард, пантера, ягуарунди  — кошачьи".into())),
    Spec = ProblemSpec::FindWord(14),
    Output = "ягуар"
}

Test! {
    Name = problem10309,
    Input = (16, InputText::ConcreteText("D, Io, Ada, Java, Swift, Python, ColdFusion  — языки программирования".into())),
    Spec = ProblemSpec::FindWord(10),
    Output = "Ada"
}

Test! {
    Name = problem10310,
    Input = (16, InputText::ConcreteText("Уфа, Азов, Пермь, Белово, Вологда, Камбарка, Соликамск  — города России".into())),
    Spec = ProblemSpec::FindWord(22),
    Output = "Соликамск"
}

Test! {
    Name = problem10856,
    Input = (16, InputText::ConcreteText("Бобр, белка, суслик, мышовка, выхухоль, тушканчик  — млекопитающие".into())),
    Spec = ProblemSpec::FindWord(16),
    Output = "суслик"
}

Test! {
    Name = problem10857,
    Input = (16, InputText::ConcreteText("Лось, хомяк, косуля, кенгуру, капибара, бинтуронг, гиппопотам  — животные".into())),
    Spec = ProblemSpec::FindWord(22),
    Output = "бинтуронг"
}

Test! {
    Name = problem10865,
    Input = (16, InputText::ConcreteText("Врач, юрист, акушер, инженер, архивист, кардиолог  — профессии".into())),
    Spec = ProblemSpec::FindWord(16),
    Output = "акушер"
}

Test! {
    Name = problem10866,
    Input = (16, InputText::ConcreteText("Репа, тыква, огурец, артишок, патиссон, картофель  — овощи".into())),
    Spec = ProblemSpec::FindWord(22),
    Output = "картофель"
}

Test! {
    Name = problem10867,
    Input = (16, InputText::ConcreteText("айва, хурма, яблоко, гуарана, апельсин, мангостан  — фрукты".into())),
    Spec = ProblemSpec::FindWord(20),
    Output = "апельсин"
}

Test! {
    Name = problem12850,
    Input = (16, InputText::ConcreteText("Як, тар, лама, окапи, пекари, бегемот, антилопа, бабирусса, бородавочник относятся к диким парнокопытным животным".into())),
    Spec = ProblemSpec::FindWord(16),
    Output = "пекари"
}

Test! {
    Name = problem18169,
    Input = (16, InputText::TextInfo {
        pages: 2,
        lines: 32,
        chars: 64,
    }),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Kb),
    Output = "8"
}

Test! {
    Name = problem18210,
    Input = (16, InputText::ConcreteText("Чиж, грач, стриж, гагара, пингвин, ласточка, жаворонок, свиристель, буревестник, вертиголовка  — птицы".into())),
    Spec = ProblemSpec::FindWord(18),
    Output = "пингвин"
}

Test! {
    Name = problem18225,
    Input = (16, InputText::ConcreteText("Чиж, грач, стриж, гагара, пингвин, ласточка, жаворонок, свиристель, буревестник, вертиголовка  — птицы".into())),
    Spec = ProblemSpec::FindWord(12),
    Output = "грач"
}

Test! {
    Name = problem18255_2,
    Input = (16, InputText::TextInfo {
        pages: 10,
        lines: 32,
        chars: 48,
    }),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Kb),
    Output = "30"
}

Test! {
    Name = problem18270,
    Input = (16, InputText::ConcreteText("Слух обо мне пройдет по всей Руси великой.".into())),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Bytes),
    Output = "84"
}

Test! {
    Name = problem18285,
    Input = (16, InputText::ConcreteText("Я к вам пишу  — чего же боле? Что я могу еще сказать?".into())),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Bytes),
    Output = "104"
}

Test! {
    Name = problem32091,
    Input = (16, InputText::ConcreteText("Алый, синий, фуксия, красный, янтарный, оранжевый, фиолетовый, канареечный, баклажановый  — цвета".into())),
    Spec = ProblemSpec::FindWord(12),
    Output = "Алый"
}

Test! {
    Name = problem10318,
    Input = (32, InputText::ConcreteText("Айва, Алыча, Генипа, Гуарана, Курбарил, Мангостан  — фрукты".into())),
    Spec = ProblemSpec::FindWord(36),
    Output = "Гуарана"
}

Test! {
    Name = problem10319,
    Input = (32, InputText::ConcreteText("Мята, тыква, фасоль, артишок, патиссон, лагенария  — овощи".into())),
    Spec = ProblemSpec::FindWord(28),
    Output = "тыква"
}

Test! {
    Name = problem10320,
    Input = (32, InputText::ConcreteText("Эри, Айыр, Гурон, Восток, Онтарио, Виннипег  — озера".into())),
    Spec = ProblemSpec::FindWord(20),
    Output = "Эри"
}

Test! {
    Name = problem10321,
    Input = (32, InputText::ConcreteText("Бай, аэта, волоф, кереки, киргизы, норвежцы  — народы".into())),
    Spec = ProblemSpec::FindWord(32),
    Output = "кереки"
}

Test! {
    Name = problem10322,
    Input = (32, InputText::ConcreteText("Врач, актер, акушер, генетик, издатель, кардиолог  — профессии".into())),
    Spec = ProblemSpec::FindWord(44),
    Output = "кардиолог"
}

Test! {
    Name = problem10858,
    Input = (32, InputText::ConcreteText("Ель, кедр, сосна, кипарис, лиственница, можжевельник  — хвойные растения".into())),
    Spec = ProblemSpec::FindWord(20),
    Output = "Ель"
}

Test! {
    Name = problem10859,
    Input = (32, InputText::ConcreteText("Нил, Амур, Волга, Ангара, Макензи, Амазонка  — реки".into())),
    Spec = ProblemSpec::FindWord(32),
    Output = "Ангара"
}

Test! {
    Name = problem10860,
    Input = (32, InputText::ConcreteText("ерш, скат, окунь, карась, камбала, долгопер  — рыбы".into())),
    Spec = ProblemSpec::FindWord(36),
    Output = "камбала"
}

Test! {
    Name = problem10864,
    Input = (32, InputText::ConcreteText("Уфа, Ухта, Тверь, Ростов, Вологда, Камбарка, Астрахань — города России".into())),
    Spec = ProblemSpec::FindWord(36),
    Output = "Вологда"
}

// ------------------------------------------------------------------------------------------------

Test! {
    Name = problem18184,
    Input = (8, InputText::TextInfo {
        pages: 8,
        lines: 40,
        chars: 48,
    }),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Kb),
    Output = "15"
}

Test! {
    Name = problem18240,
    Input = (16, InputText::TextInfo {
        pages: 20,
        lines: 40,
        chars: 48,
    }),
    Spec = ProblemSpec::CalcTextSize(DataSizeUnit::Kb),
    Output = "75"
}
