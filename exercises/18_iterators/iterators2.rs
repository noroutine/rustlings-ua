// У цій вправі ви вивчите деякі унікальні переваги, які можуть запропонувати
// ітератори.

// TODO: Доповніть функцію `capitalize_first`.
// "привіт" -> "Привіт"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO: Застосуйте функцію `capitalize_first` до зріза рядкових зрізів.
// Поверніть вектор рядків.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
}

// TODO: Застосуйте функцію `capitalize_first` знову до зріза рядкових
// зрізів. Поверніть один рядок.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("привіт"), "Привіт");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["привіт", "світ"];
        assert_eq!(capitalize_words_vector(&words), ["Привіт", "Світ"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
