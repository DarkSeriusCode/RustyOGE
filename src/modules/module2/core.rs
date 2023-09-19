use super::types::Codes;

/// Алгоритм, декодирующий строку всеми возможными способами
/// по таблице символов.
pub fn decode(codes: &Codes, string: &String) -> Vec<String> {
    let string_len = string.chars().count();

    if let Some(letter) = codes.get(string) {
        // Пытаемся представить stirng не как один код, а как несколько
        let mut additional_letters = vec![];
        for i in 1..=string_len {
            if let Some(_) = codes.get(&string[..i].to_string()) {
                additional_letters.extend(decode(&codes, &string[i..].to_string()));
            }
        }
        // Добавляем к символу все дополнительные разшифровки
        let mut res = vec![letter.to_string()];
        res.extend(additional_letters);
        return res;

    } else {
        let mut decoded_strings: Vec<String> = vec![];
        let mut buffer = String::new();
        for i in 0..string_len {
            buffer.push_str(&string.chars().nth(i).unwrap().to_string());

            // Если совпадение с кодом в таблице - декодируем оставшуюся строку
            if let Some(letter) = codes.get(&buffer) {
                for s in decode(codes, &string[i+1..].to_string()) {
                    decoded_strings.push(letter.clone() + &s);
                }
            }
        }
        return decoded_strings;
    }
}
