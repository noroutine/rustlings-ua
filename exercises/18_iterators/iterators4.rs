fn factorial(num: u64) -> u64 {
    // TODO: Доповніть цю функцію, щоб вона повертала факторіал `num`, який
    // визначається як `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Не використовуйте:
    // - ранні повернення (явне використання ключового слова `return`)
    // Намагайтеся не використовувати:
    // - імперативні цикли (for/while)
    // - додаткові змінні
    // За бажанням, для ускладнення задачі, не використовуйте:
    // - рекурсію
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
