fn main() {
    let my_first_initial = 'Є';
    if my_first_initial.is_alphabetic() {
        println!("Алфавітний!");
    } else if my_first_initial.is_numeric() {
        println!("Числовий!");
    } else {
        println!("Ні алфавітний, ні числовий!");
    }

    // Приклад з емодзі.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Алфавітний!");
    } else if your_character.is_numeric() {
        println!("Числовий!");
    } else {
        println!("Ні алфавітний, ні числовий!");
    }
}
