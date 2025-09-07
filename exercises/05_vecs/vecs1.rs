fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Масив (Array)

    // TODO: Створіть вектор з назвою `v`, який містить точно такі ж елементи, як і в масиві `a`.
    // Використайте макрос вектору.
    // let v = ???;

    (a, v)
}

fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
