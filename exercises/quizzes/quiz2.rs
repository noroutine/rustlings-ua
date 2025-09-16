// Це квіз для наступних розділів:
// - Рядки (Strings)
// - Вектори (Vecs)
// - Семантика переміщення (Move semantics)
// - Модулі (Modules)
// - Перелічення (Enums)
//
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

    // TODO: Доповніть функцію, як описано вище.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Ви можете експериментувати тут за бажанням.
}

#[cfg(test)]
mod tests {
    // TODO: Що нам потрібно імпортувати, щоб `transformer` був у області видимості?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
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
