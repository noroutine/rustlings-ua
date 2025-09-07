fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    // TODO: Виправте помилки компілятора, лише переставивши рядки в тесті.
    // Не додавайте, не змінюйте і не видаляйте жодного рядка.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
