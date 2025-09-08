// Потрібно визначити кошик фруктів у вигляді хеш-мапи. Ключ
// представляє назву фрукта, а значення - скільки одиниць цього
// конкретного фрукта є в кошику. Ви повинні покласти принаймні 3 різні
// типи фруктів (наприклад, яблуко, банан, манго) в кошик і загальна кількість
// усіх фруктів повинна бути принаймні 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Оголошуємо хеш-мапу.
    let mut basket = HashMap::new();

    // Два банани вже додані для вас :)
    basket.insert(String::from("банан"), 2);

    // Покладіть більше фруктів у ваш кошик.
    basket.insert(String::from("яблуко"), 3);
    basket.insert(String::from("манго"), 1);

    basket
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
