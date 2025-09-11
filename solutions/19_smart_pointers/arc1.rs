// У цій вправі ми маємо `Vec` з `u32` під назвою `numbers` зі значеннями
// від 0 до 99. Ми хочемо використовувати цей набір чисел у 8
// різних потоках (threads) одночасно. Кожний потік буде отримувати суму
// кожного восьмого значення зі зміщенням.
//
// Перший потік (зміщення 0) підсумовуватиме 0, 8, 16, …
// Другий потік (зміщення 1) підсумовуватиме 1, 9, 17, …
// Третій потік (зміщення 2) підсумовуватиме 2, 10, 18, …
// …
// Восьмий потік (зміщення 7) підсумовуватиме 7, 15, 23, …
//
// Кожен потік повинен володіти вказівником підрахунку посилань (reference-counting pointer) 
// на вектор чисел. Але `Rc` не є потокобезпечним (thread-safe). Тому нам потрібно використовувати `Arc`.
//
// Не відволікайтесь на те, як створюються та з'єднуються потоки. Ми попрактикуємо
// це пізніше у вправах про потоки.

// Не змінюйте рядки нижче.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);
    //                   ^^^^^^^^^^^^^^^^^

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

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
