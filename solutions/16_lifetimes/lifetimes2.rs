fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("дуже довгий рядок");
    // Рішення 1: Ви можете перемістити `string2` за межі внутрішнього блоку,
    // щоб вона не була знищена перед виведенням.
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("Найдовший рядок - це '{result}'");
    // `string2` знищується в кінці функції.

    // =========================================================================

    let string1 = String::from("дуже довгий рядок");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Рішення 2: Ви можете перемістити виведення у внутрішній блок,
        // щоб воно виконалося перед знищенням `string2`.
        println!("Найдовший рядок - це '{result}'");
        // `string2` знищується тут (кінець внутрішньої області видимості).
    }
}
