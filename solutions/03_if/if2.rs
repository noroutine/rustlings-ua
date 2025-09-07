fn picky_eater(food: &str) -> &str {
    if food == "полуничка" {
        "Ммммм!"
    } else if food == "картопелька" {
        "Я зможу..."
    } else {
        "Ні, тільки не це!"
    }
}

fn main() {
    // Тут можете за бажанням поекспериментувати.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("полуничка"), "Ммммм!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("картопелька"), "Я зможу...");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("броколі"), "Ні, тільки не це!");
        assert_eq!(picky_eater("желейні ведмедики"), "Ні, тільки не це!");
        assert_eq!(picky_eater("буквально будь-що"), "Ні, тільки не це!");
    }
}