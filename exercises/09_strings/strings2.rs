// TODO: Виправте помилку компілятора у функції `main`, не змінюючи цю функцію.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "зелений" || attempt == "синій" || attempt == "червоний"
}

fn main() {
    let word = String::from("зелений"); // Не змінюйте цей рядок.

    if is_a_color_word(word) {
        println!("Я знаю цей колір!");
    } else {
        println!("Цей колір мені невідомий.");
    }
}
