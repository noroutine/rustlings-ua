use std::mem;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // `unwrap` для `Option` після перевірки, що він є `None`, призведе до паніки.
    // Використовуйте `if-let` замість цього.
    if let Some(value) = my_option {
        println!("{value}");
    }

    // Бракувало коми.
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("Мій масив! Ось він: {my_arr:?}");

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // `resize` змінює вектор замість повернення нового.
    // `resize(0, …)` очищає вектор, тому краще використовувати `clear`.
    my_empty_vec.clear();
    println!("Цей вектор порожній, бачиш? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Використовуйте `mem::swap` для правильного обміну двох значень.
    mem::swap(&mut value_a, &mut value_b);
    println!("значення a: {value_a}; значення b: {value_b}");
}
