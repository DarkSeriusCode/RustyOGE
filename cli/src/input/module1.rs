use rusty_oge::module1;
use rusty_oge::utils::{Validated, data_size::DataSizeUnit};

use crate::errors::{CLIResult, CLIError};
use super::input_utils::*;

pub fn get_input() -> CLIResult<module1::InputData> {
    let bits_in_char: usize = input("Сколькими битами кодируется один символ:")?;
    let text = get_text()?;
    let is_concrete_text_given = if let module1::InputText::ConcreteText(_) = text { true }
                                 else { false };
    let spec = get_spec(is_concrete_text_given)?;

    let input_data = module1::InputData::new(bits_in_char, text, spec);

    if let Err(validation_error_text) = input_data.valid() {
        return Err(CLIError::InvalidInputData(validation_error_text));
    }

    Ok(input_data)
}

fn get_spec(is_concrete_text_given: bool) -> CLIResult<module1::ProblemSpec> {
    if is_concrete_text_given {
        let word_size: usize = input("На сколько байт изменился размер текста:")?;
        return Ok(module1::ProblemSpec::FindWord(word_size));
    }

    let data_unit: DataSizeUnit = *choose("В каких единицах посчитать размер?",
        &[("Байты",   &DataSizeUnit::Bytes),
        ("Килобайты", &DataSizeUnit::Kb),
        ("Мегабайты", &DataSizeUnit::Mb)]
    )?;
    return Ok(module1::ProblemSpec::CalcTextSize(data_unit));
}

fn get_text() -> CLIResult<module1::InputText> {
    let is_it_concrete_text = *choose(
        "В задаче дан текст или инф-ция о кол-ве страниц, строк и т.д:",
        &[("Текст", &true), ("Информация о тексте", &false)]
    )?;

    if is_it_concrete_text {
        let text: String = input("Введите текст:")?;
        return Ok(module1::InputText::ConcreteText(text));
    }

    let pages: usize = input("Количество страниц:")?;
    let lines: usize = input("Количество строк на странице:")?;
    let chars: usize = input("Количество символов в строке:")?;
    return Ok(module1::InputText::TextInfo { pages, lines, chars });
}
