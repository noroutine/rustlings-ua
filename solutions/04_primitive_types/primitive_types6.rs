fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Синтаксис індексації кортежів.
        let second = numbers.1;

        assert_eq!(second, 2, "Це не другий елемент кортежу!");
    }
}
