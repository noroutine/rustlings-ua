// Ось ще кілька легких виправлень Clippy, щоб ви могли побачити його корисність 📎
// TODO: Виправте всі попередження Clippy.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Припустіть, що ви не знаєте значення `my_option`.
    // У випадку `Some`, ми хочемо вивести його значення.
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("Мій масив! Ось він: {my_arr:?}");

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("Цей вектор порожній, бачиш? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Давайте поміняємо ці два місцями!
    value_a = value_b;
    value_b = value_a;
    println!("значення a: {value_a}; значення b: {value_b}");
}
