// Ми збираємо різні фрукти для печіння смачного фруктового торта. Для цього
// у нас є кошик, який ми представимо у вигляді хеш-мапи. Ключ
// представляє назву кожного фрукта, який ми збираємо, а значення представляє кількість
// цього конкретного фрукта. Три типи фруктів -
// Яблуко (4), Манго (2) і Лічі (5) вже є в кошику хеш-мапи. Ви
// повинні додати фрукти до кошика так, щоб кожного виду було принаймні один і
// загалом більше 11 - у нас багато ротів, які потрібно нагодувати. Вам заборонено
// додавати більше фруктів, які вже є в кошику (Яблуко,
// Манго і Лічі).

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,    // Яблуко
    Banana,   // Банан
    Mango,    // Манго
    Lychee,   // Лічі
    Pineapple, // Ананас
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Вставте нові фрукти, якщо вони ще не присутні в
        // кошику. Зверніть увагу, що вам заборонено класти будь-який тип фруктів,
        // який вже присутній!
    }
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Не змінюйте цю функцію!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Тип фрукта {fruit_kind:?} не знайдено в кошику");
            };
            assert!(*amount > 0);
        }
    }
}
