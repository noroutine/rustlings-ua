fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Доповніть цю функцію, щоб вона повертала більше число!
    // Якщо обидва числа рівні, можна повернути будь-яке з них.
    // Не використовуйте:
    // - виклик іншої функції
    // - додаткові змінні
}

fn main() {
    // Тут можете за бажанням поекспериментувати.
}

// Поки що не звертайте на це увагу :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}