use crate::utils::{self, Validated, data_size::DataSizeUnit};

/// Детали решения задания. Указывает, что нужно сделать в данном тексте
#[derive(Debug, Clone)]
pub enum ProblemSpec {
    /// В задаче требуется найти слово, после добавления/вычёркивания текст стал изменился на
    /// `usize` байт
    FindWord(usize),
    /// В задаче требуется найти размер текста в `DataSizeUnit`
    CalcTextSize(DataSizeUnit),
}

// ------------------------------------------------------------------------------------------------

/// Данный в задаче текст. Текстом может считаться строка или _информация_ о количестве страниц,
/// строк и символов
#[derive(Debug, Clone)]
pub enum InputText {
    /// В задаче просто дана строка текста
    ConcreteText(String),
    /// В задаче дано кол-во страниц, строк и символов
    TextInfo {
        pages: usize,
        lines: usize,
        chars: usize
    },
}

impl InputText {
    pub fn chars_count(&self) -> usize {
        match self {
            Self::ConcreteText(txt)                => utils::normalize_text(txt).chars().count(),
            Self::TextInfo { pages, lines, chars } => pages * lines * chars,
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Входные данные задачи.
#[derive(Debug, Clone)]
pub struct InputData {
    /// Сколькими **битами** кодируется один символ.
    pub bits_in_char: usize,
    /// Данный в задаче текст.
    pub text: InputText,
    /// Детали решения задачи.
    pub spec: ProblemSpec,
}

impl InputData {
    pub fn new(bits_in_char: usize, text: InputText, spec: ProblemSpec) -> Self {
        Self { bits_in_char, text, spec }
    }
}

impl Validated for InputData {
    fn valid(&self) -> Result<(), String> {
        if let ProblemSpec::FindWord(_) = self.spec {
            if let InputText::TextInfo { .. } = self.text {
                return Err("Cannot to find word if `text` is not specified".into());
            }
        }
        Ok(())
    }
}
