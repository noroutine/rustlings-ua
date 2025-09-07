// TODO: Виправте помилки в цій функції
fn picky_eater(food: &str) -> &str {
    if food == "полуничка" {
        "Ммммм!"
    } else {
        1
    }
}

fn main() {
    // Тут можете за бажанням поекспериментувати.
}

// TODO: Прочитайте код тестів, щоб зрозуміти бажану поведінку функції
// Зробіть так, щоб усі тести пройшли успішно, але не змінюйте тести
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Це значить що виклик `picky_eater` з аргументом "полуничка" має повернути "Ммммм!".
        assert_eq!(picky_eater("полуничка"), "Ммммм!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("картопелька"), "Я зможу...");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("броколі"), "Ні, тільки не це!");
        assert_eq!(picky_eater("желейні ведмедики"), "Ні, тільки не це!");
        assert_eq!(picky_eater("буквально будь-що"), "Ні, тільки не це!");
    }
}
