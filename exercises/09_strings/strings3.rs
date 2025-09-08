fn trim_me(input: &str) -> &str {
    // TODO: Видаліть пробіли з обох кінців рядка.
}

fn compose_me(input: &str) -> String {
    // TODO: Додайте " світ!" до рядка! Є декілька способів це зробити.
}

fn replace_me(input: &str) -> String {
    // TODO: Замініть "машини" в рядку на "кульки".
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
