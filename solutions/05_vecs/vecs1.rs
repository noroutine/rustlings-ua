fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Масив (Array)

    // Використали макрос `vec!`.
    let v = vec![10, 20, 30, 40];

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
