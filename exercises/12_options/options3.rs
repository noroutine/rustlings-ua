#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Виправте помилку компілятора, додавши щось до цього оператора match.
    match optional_point {
        Some(p) => println!("Координати: {},{}", p.x, p.y),
        _ => panic!("Немає відповідності!"),
    }

    println!("{optional_point:?}"); // Не змінюйте цей рядок.
}
