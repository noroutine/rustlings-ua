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
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Перевірити ширину
        assert_eq!(rect.height, 20); // Перевірити висоту
    }

    #[test]
    #[should_panic] // Додано цей атрибут для перевірки, що тест panic.
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // Додано цей атрибут для перевірки, що тест panic.
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
