// У цій вправі нам дано `Vec` з `u32`, названий `numbers`, зі значеннями
// від 0 до 99. Ми б хотіли використати цей набір чисел в 8
// різних потоках (threads) одночасно. Кожен потік отримає суму
// кожного восьмого значення з зміщенням.
//
// Перший потік (зміщення 0) підсумує 0, 8, 16, …
// Другий потік (зміщення 1) підсумує 1, 9, 17, …
// Третій потік (зміщення 2) підсумує 2, 10, 18, …
// …
// Восьмий потік (зміщення 7) підсумує 7, 15, 23, …
//
// Кожен потік має володіти показчиком із підрахунком посилань на вектор
// чисел. Але `Rc` не є потокобезпечним. Отже, нам потрібно використати `Arc`.
//
// Не відволікайтеся на те, як потоки створюються та об'єднуються. Ми попрактикуємо
// це пізніше у вправах про потоки (threads).

// Не змінюйте рядки нижче.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Визначте `shared_numbers`, використовуючи `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Визначте `child_numbers`, використовуючи `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Сума по зміщенню {offset} є {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
