fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Отримайте зріз (slice) з назвою `nice_slice` з масиву `a` так, щоб тест пройшов.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
