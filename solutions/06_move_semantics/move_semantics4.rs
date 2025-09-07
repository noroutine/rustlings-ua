fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` використовується тут.
        y.push(42);
        // Незмінюване посилання `y` більше не використовується,
        // тому можна створити нове посилання.
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
