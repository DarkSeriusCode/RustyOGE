/// Ищет в тексте первое слово, сумма длины которого с длиной не алфавитных символов после него
/// равна `len`.
/// Например: `find_word_with_len("Слово, слог", 7)` вернёт "Слово", т.к "Слово" (5 букв) + ", " =
/// 7
pub fn find_word_with_len(text: &str, len: usize) -> Option<String> {
    let mut begin_idx = 0;

    while begin_idx < text.chars().count() {
        let word: String = text.chars().skip(begin_idx).take_while(|c| c.is_alphabetic()).collect();
        let word_len = word.chars().count();
        begin_idx += word_len;

        let punct_len = text.chars().skip(begin_idx).take_while(|c| !c.is_alphabetic()).count();
        begin_idx += punct_len;

        if word_len <= len && word_len + punct_len >= len {
            return Some(word.to_string());
        }
    }
    None
}
