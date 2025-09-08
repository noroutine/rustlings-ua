#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Рішення 1: Зіставлення з `Option` (не `&Option`), але без переміщення
    // з варіанта `Some`.
    match optional_point {
        Some(ref p) => println!("Координати: {},{}", p.x, p.y),
        //   ^^^ додано
        _ => panic!("Немає відповідності!"),
    }

    // Рішення 2: Зіставлення з посиланням (`&Option`), додавши `&` перед
    // `optional_point`.
    match &optional_point {
        //^ додано
        Some(p) => println!("Координати: {},{}", p.x, p.y),
        _ => panic!("Немає відповідності!"),
    }

    println!("{optional_point:?}");
}
