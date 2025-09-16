// Давайте побудуємо невелику машину у вигляді функції. Як вхідні дані, ми
// подаємо список рядків і команд. Ці команди визначають, яку дію
// буде застосовано до рядка. Це може бути:
// - Перевести рядок у верхній регістр
// - Обрізати рядок
// - Додати "bar" до рядка вказану кількість разів
//
// Точна форма цього буде наступною:
// - Вхід буде вектор 2-елементних кортежів,
//   перший елемент - рядок, другий - команда.
// - Вихідний елемент буде вектор рядків.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Розв'язок з циклом. Подивіться на `transformer_iter` для версії
    // з ітераторами.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // Створюємо новий рядок.
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // Додаємо новий рядок до вихідного вектора.
            output.push(new_string);
        }

        output
    }

    // Еквівалентно до `transform`, але використовує ітератор замість циклу для
    // порівняння. Не хвилюйтеся, ми попрактикуємо ітератори пізніше ;)
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // Ви можете експериментувати тут за бажанням.
}

#[cfg(test)]
mod tests {
    // Імпортуємо `transformer`.
    use super::my_module::transformer;

    use super::Command;
    use super::my_module::transformer_iter;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
            let input = vec![
                ("hello".to_string(), Command::Uppercase),
                (" all roads lead to rome! ".to_string(), Command::Trim),
                ("foo".to_string(), Command::Append(1)),
                ("bar".to_string(), Command::Append(5)),
            ];
            let output = transformer(input);

            assert_eq!(
                output,
                [
                    "HELLO",
                    "all roads lead to rome!",
                    "foobar",
                    "barbarbarbarbarbar",
                ]
            );
        }
    }
}
