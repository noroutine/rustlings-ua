// Обчислює степінь 2, використовуючи бітовий зсув.
// `1 << n` еквівалентно "2 у степені n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
    }
}
