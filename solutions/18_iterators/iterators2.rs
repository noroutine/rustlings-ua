// У цій вправі ви вивчите деякі унікальні переваги, які можуть запропонувати
// ітератори.

// "привіт" -> "Привіт"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// Застосуйте функцію `capitalize_first` до зрізу рядкових зрізів.
// Поверніть вектор рядків.
// ["привіт", "світ"] -> ["Привіт", "Світ"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Застосуйте функцію `capitalize_first` знову до зрізу рядкових
// зрізів. Поверніть один рядок.
// ["привіт", " ", "світ"] -> "Привіт Світ"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
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
        let words = vec!["привіт", " ", "світ"];
        assert_eq!(capitalize_words_string(&words), "Привіт Світ");
    }
}
