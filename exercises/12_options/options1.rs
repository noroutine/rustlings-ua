// Ця функція повертає, скільки морозива залишилося в холодильнику.
// Якщо це до 22:00 (24-годинна система), то залишилося 5 кульок. О 22:00
// хтось з'їдає все, тому морозива не залишається (значення 0). Поверніть `None`, якщо
// `hour_of_day` більше за 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Завершіть тіло функції.
}

fn main() {
    // Ви можете тут експериментувати, якщо бажаєте.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Виправте цей тест. Як отримати значення, яке міститься в
        // Option?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Не змінюйте цей рядок.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
