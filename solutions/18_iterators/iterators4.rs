// Представлено 3 можливі рішення.

// З циклом `for` та змінюваною змінною.
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// Еквівалентний `factorial_for`, але коротший і без циклу `for` та
// змінюваних змінних.
fn factorial_fold(num: u64) -> u64 {
    // Випадок num==0: ітератор 2..=0 порожній
    //                  -> Повертається початкове значення `fold`, яким є 1.
    // Випадок num==1: ітератор 2..=1 теж порожній
    //                  -> Повертається початкове значення 1.
    // Випадок num==2: ітератор 2..=2 містить один елемент
    //                  -> Початкове значення 1 помножується на 2 і результат
    //                     повертається.
    // Випадок num==3: ітератор 2..=3 містить 2 елементи
    //                  -> Обчислюється 1 * 2, потім результат 2 помножується на
    //                     другий елемент 3, тому повертається результат 6.
    // І так далі…
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// Еквівалентний `factorial_fold`, але з вбудованим методом, який пропонує
// Clippy.
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
