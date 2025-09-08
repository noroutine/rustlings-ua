fn current_favorite_color() -> String {
    // Еквівалентно `String::from("синій")`
    "синій".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("Мій найулюбленіший колір зараз: {answer}");
}
