fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(input: &str) -> String {
    // Макрос `format!` має той самий синтаксис, що і `println!`, але повертає
    // рядок замість виводити його на термінал.
    // Еквівалентно `input.to_string() + " світ!"`
    format!("{input} світ!")
}

fn replace_me(input: &str) -> String {
    input.replace("машини", "кульки")
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Привіт!     "), "Привіт!");
        assert_eq!(trim_me("  Як справи?"), "Як справи?");
        assert_eq!(trim_me("   Привіт!  "), "Привіт!");
        assert_eq!(trim_me("Привіт!"), "Привіт!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Привіт"), "Привіт світ!");
        assert_eq!(compose_me("Бувай"), "Бувай світ!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("Я думаю, машини круті"),
            "Я думаю, кульки круті",
        );
        assert_eq!(
            replace_me("Я люблю дивитись на машини"),
            "Я люблю дивитись на кульки",
        );
    }
}
