fn animal_habitat(animal: &str) -> &str {
    // TODO: Виправте помилку компілятора в цьому виразі
    let identifier = if animal == "краб" {
        1
    } else if animal == "ховрах" {
        2.0
    } else if animal == "змія" {
        3
    } else {
        "Невідомо"
    };

    // Не змінюйте цей вираз
    if identifier == 1 {
        "Пляж"
    } else if identifier == 2 {
        "Нора"
    } else if identifier == 3 {
        "Пустеля"
    } else {
        "Невідомо"
    }
}

fn main() {
    // Тут можете за бажанням поекспериментувати.
}

// Не змінюйте тести!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("ховрах"), "Нора")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("змія"), "Пустеля")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("краб"), "Пляж")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("динозавр"), "Невідомо")
    }
}
