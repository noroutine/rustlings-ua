// Це схоже на попередню вправу `from_into`. Але цього разу ми реалізуємо
// `FromStr` і повертаємо помилки замість повернення до значення
// за замовчуванням. Крім того, після реалізації `FromStr`, ви можете використовувати
// метод `parse` на рядках, щоб створити об'єкт реалізаційного типу. Ви можете
// прочитати більше про це у документації:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Ми будемо використовувати цей тип помилки для реалізації `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Неправильна кількість полів
    BadLen,
    // Порожнє поле імені
    NoName,
    // Обгорнута помилка з parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Доповніть цю реалізацію `FromStr`, щоб можна було розібрати `Person`
// з рядка в форматі "Mark,20".
// Зауважте, що вам потрібно буде розібрати компонент віку на `u8` з
// чимось на кшталт `"4".parse::<u8>()`.
//
// Кроки:
// 1. Розділіть наданий рядок за комами, які в ньому є.
// 2. Якщо операція розділення повертає менше або більше 2 елементів,
//    повернути помилку `ParsePersonError::BadLen`.
// 3. Використати перший елемент з операції розділення як ім'я.
// 4. Якщо ім'я порожнє, повернути помилку `ParsePersonError::NoName`.
// 5. Розібрати другий елемент з операції розділення на `u8` як вік.
// 6. Якщо розбор віку не вдається, повернути помилку `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
