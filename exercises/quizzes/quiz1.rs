// Це квіз для наступних розділів:
// - Змінні (Variables)
// - Функції (Functions)
// - Умовні конструкції (If)
//
// Мері купує яблука. Ціна яблука обчислюється наступним чином:
// - Одне яблуко коштує 2 растбаки.
// - Однак, якщо Мері купує більше 40 яблук, ціна кожного яблука в
//   всьому замовленні зменшується до лише 1 растбака!

// TODO: Напишіть функцію, яка обчислює ціну замовлення яблук за заданою
// кількістю.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // Ви можете експериментувати тут за бажанням.
}

// Не змінюйте тести!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
