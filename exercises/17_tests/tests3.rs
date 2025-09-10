struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Не змінюйте цю функцію.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Повернути `Result` було б тут краще. Але ж ми хочемо вивчити,
            // як тестувати функції, які можуть панікувати (panic).
            panic!("Ширина та висота прямокутника мають бути додатнімі числами");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Тут ви можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: Цей тест має перевірити, чи має прямокутник розмір,
        // який ми передаємо його конструктору.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Перевірити ширину
        assert_eq!(todo!(), 20); // Перевірити висоту
    }

    // TODO: Цей тест має перевірити, чи програма panic, коли ми пробуємо
    // створити прямокутник з негативною шириною.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: Цей тест має перевірити, чи програма panic, коли ми пробуємо
    // створити прямокутник з негативною висотою.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
