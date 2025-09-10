trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Реалізуйте трейт `AppendBar` для вектора рядків (strings).
// `append_bar` має додати рядок "Bar" у вектор.

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
