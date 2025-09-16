// Мері купує яблука. Ціна яблука обчислюється наступним чином:
// - Одне яблуко коштує 2 растбаки.
// - Однак, якщо Мері купує більше 40 яблук, ціна кожного яблука в
//   всьому замовленні зменшується до лише 1 растбака!

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

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
