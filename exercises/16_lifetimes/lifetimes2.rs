// Не змінюйте цю функцію.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Виправте помилку компілятора, перемістивши один рядок.

    let string1 = String::from("дуже довгий рядок");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("Найдовший рядок - це '{result}'");
}
