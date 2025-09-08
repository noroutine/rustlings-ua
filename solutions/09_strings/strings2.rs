fn is_a_color_word(attempt: &str) -> bool {
    attempt == "зелений" || attempt == "синій" || attempt == "червоний"
}

fn main() {
    let word = String::from("зелений");

    if is_a_color_word(&word) {
        //             ^ додано для отримання `&String`, яке автоматично
        //               приводиться до `&str` компілятором.
        println!("Я знаю цей колір!");
    } else {
        println!("Цей колір мені невідомий.");
    }
}
