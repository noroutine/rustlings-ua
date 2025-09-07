fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Помножте кожен елемент зрізу `input` на 2 і додайте його до
        // вектору `output`.
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Приклад збору (collecting) вектора після відображення.
    // Ми відображаємо кожен елемент зрізу `input` до його значення плюс 1.
    // Якщо вхід `[1, 2, 3]`, то вихід `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Тут ми також хочемо помножити кожен елемент зрізу `input`
    // на 2, але за допомогою ітератора та відображення (mapping), а не 
    // шляхом ручного додавання елементів до порожнього вектора.
    // Подивіться на приклад у функції `vec_map_example` вище.
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
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
