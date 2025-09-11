// Ця програма створює кілька потоків (threads), кожен з яких виконується
// щонайменше 250мс, і кожен потік повертає, скільки часу знадобилося для 
// завершення. Програма має чекати, поки всі створені потоки не завершать
// роботу і має зібрати їх повернені значення у вектор.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Потік {i} завершується");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Зіберіть результати всіх потоків у вектор `results`.
        // Використайте структуру `JoinHandle`, яку повертає `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("О, ні! Якийсь потік ще не завершився!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Потік {i} виконувався {result}мс");
    }
}
