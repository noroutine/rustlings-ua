// Продовжуючи останню вправу, ми хочемо, щоб всі потоки завершили свою
// роботу. Але цього разу створені потоки мають оновлювати
// спільне значення: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // `Arc` недостатньо, якщо вам потрібен **змінний** спільний стан.
    // Нам потрібно загорнути значення у `Mutex`.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    //                    ^^^^^^^^^^^                          ^

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Заблокуйте мутекс перед оновленням спільного значення.
            status_shared.lock().unwrap().jobs_done += 1;
            //           ^^^^^^^^^^^^^^^^
        });
        handles.push(handle);
    }

    // Очікуємо завершення всіх завдань.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Справу зроблено: {}", status.lock().unwrap().jobs_done);
    //                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
