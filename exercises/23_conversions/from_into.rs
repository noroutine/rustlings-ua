// Трейт `From` використовується для перетворень значення-у-значення. Якщо `From`
// реалізований, то реалізація `Into` надається автоматично.
// Ви можете прочитати більше про це у документації:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Ми реалізуємо трейт Default, щоб використовувати його як запасний варіант,
// коли наданий рядок не може бути перетворений на об'єкт `Person`.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Доповніть цю реалізацію `From`, щоб можна було розібрати `Person`
// з рядка в форматі "Mark,20".
// Зауважте, що вам потрібно буде розібрати компонент віку на `u8` з
// чимось на кшталт `"4".parse::<u8>()`.
//
// Кроки:
// 1. Розділіть наданий рядок за комами, які в ньому є.
// 2. Якщо операція розділення повертає менше або більше 2 елементів,
//    повернути значення за замовчуванням для `Person`.
// 3. Використати перший елемент з операції розділення як ім'я.
// 4. Якщо ім'я порожнє, повернути значення за замовчуванням для `Person`.
// 5. Розібрати другий елемент з операції розділення на `u8` як вік.
// 6. Якщо розбор віку не вдається, повернути значення за замовчуванням для `Person`.
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // Використайте функцію `from`.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Оскільки `From` реалізований для Person, ми можемо використовувати `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
