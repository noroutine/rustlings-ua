// Продовжуючи останню вправу, ми хочемо, щоб всі потоки завершили свою
// роботу. Але цього разу створені потоки мають оновлювати
// спільне значення: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` недостатньо, якщо вам потрібен **змінний** спільний стан.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: Ви маєте щось зробити перед оновленням спільного значення.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Чекаємо завершення всіх робіт (jobs).
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Виведіть значення `JobStatus.jobs_done`.
    println!("Справу зроблено: {}", todo!());
}
