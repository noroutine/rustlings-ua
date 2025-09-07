#![allow(clippy::ptr_arg)]

// TODO: Виправте помилки компілятора, не змінюючи нічого, окрім додавання або
// видалення посилань (символ `&`).

// Не повинна забирати володіння (ownership)
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Повинна забирати володіння (ownership)
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust неймовірний!".to_string();

    get_char(data);

    string_uppercase(&data);
}
