fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "краб" {
        1
    } else if animal == "ховрах" {
        2
    } else if animal == "змія" {
        3
    } else {
        // Будь-яке інше значення
        4
    };

    // Замість такого ідентифікатора, в Rust використовуютьс перелічення (enums)
    // Але ми до них ще не дійшли.
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
