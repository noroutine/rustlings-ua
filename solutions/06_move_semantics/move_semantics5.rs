#![allow(clippy::ptr_arg)]

// Функція запозичує змінну замість забирати володіння.
// Рекомендується використовувати `&str` замість `&String` тут. Але поки цього
// достатньо, оскільки ми ще не опрацювали рядки.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Забирає володіння замість запозичення.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust неймовірний!".to_string();

    get_char(&data);

    string_uppercase(data);
}
