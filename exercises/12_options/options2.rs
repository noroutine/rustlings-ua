fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "растівки";
        let optional_target = Some(target);

        // TODO: Зробіть це оператором if-let, чиє значення є `Some`.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Зробіть це оператором while-let. Пам'ятайте, що `Vec::pop()`
        // додає ще один шар `Option`. Ви можете робити вкладене зіставлення з шаблоном
        // в операторах if-let і while-let.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
