fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Приклад збору вектора після відображення.
    // Ми відображаємо кожен елемент зрізу `input` до його значення плюс 1.
    // Якщо вхід `[1, 2, 3]`, то вихід `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // Ми глибше зануримось в ітератори, але поки це все, що вам
    // потрібно було зробити!
    // Примітка для допитливих: Цей спосіб ефективніший, оскільки заздалегідь
    // виділяє достатню памʼять. Це можна зробити і вручну в `vec_loop`,
    // використовуючи `Vec::with_capacity(input.len())` замість `Vec::new()`.
    input.iter().map(|element| 2 * element).collect()
}

fn main() {
    // Тут ви можете за бажанням експериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
