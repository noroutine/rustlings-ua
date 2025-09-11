// Для операцій з елементами в колекції ітератори (iterators) є дуже
// важливими. Цей модуль допомагає вам ознайомитися зі структурою використання
// ітератора та як обробляти елементи в ітерованій колекції.

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["банан", "кремове яблуко", "авокадо", "персик", "малина"];

        // TODO: Створіть ітератор над масивом.
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"банан"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Замініть `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"авокадо"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Замініть `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"малина"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Замініть `todo!()`
    }
}
